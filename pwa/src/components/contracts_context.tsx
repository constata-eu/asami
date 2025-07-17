import asamiABI from "../abi/Asami.json";
import docABI from "../abi/Doc.json";
import { rLogin } from "../lib/rLogin";
import { ethers } from "ethers";
import { useSafeSetState } from "react-admin";
import { createContext, useContext } from "react";
import { HttpError } from "react-admin";
import { Settings } from "../settings";
import { Client, XOConnectProvider } from "xo-connect";

const ContractsContext = createContext(null);

export const ContractsProvider = ({ children }) => {
  const [values, setValues] = useSafeSetState(null);

  const require_signer = async (expected, actual, disconnect) => {
    if (expected && expected.toLowerCase() != actual.toLowerCase()) {
      await disconnect;
      throw { code: "WRONG_SIGNER", expected, actual };
    }
  };

  const contracts = async (expected_signer, isEmbedded) => {
    if (values) {
      await require_signer(
        expected_signer,
        values.signer.address,
        values.disconnect,
      );
      return values;
    }

    const config = await (await fetch(`${Settings.apiDomain}/config`)).json();

    const connectEmbedded = () => {
      return {
        provider: new XOConnectProvider(),
        disconnect: async () => {},
      };
    };

    const { provider, disconnect } = isEmbedded
      ? connectEmbedded()
      : await rLogin.connect();

    provider.on("accountsChanged", async () => {
      await disconnect;
      setValues(null);
    });
    provider.on("disconnect", () => {
      setValues(null);
    });
    const ethersProvider = new ethers.BrowserProvider(provider);

    const signer = await ethersProvider.getSigner(0);

    await require_signer(expected_signer, signer.address, disconnect);

    const asamiAddress = config.contractAddress;
    const docAddress = config.docContractAddress;
    const asami = new ethers.Contract(asamiAddress, asamiABI.abi, signer);
    const doc = new ethers.Contract(docAddress, docABI, signer);
    const newVals = {
      doc,
      asami,
      asamiAddress,
      docAddress,
      signer,
      provider,
      disconnect,
    };
    setValues(newVals);
    return newVals;
  };

  const signLoginMessage = async (
    isEmbedded: boolean,
    message = "Login to Asami",
    expectedSigner = null,
  ) => {
    try {
      const { signer, provider } = await contracts(expectedSigner, isEmbedded);

      const address = await signer.getAddress();

      const typedData = {
        domain: {
          name: "Asami",
          version: "1",
          chainId: Settings.rsk.chainId,
        },
        message: {
          content: message,
        },
        primaryType: "Acceptance",
        types: {
          EIP712Domain: [
            { name: "name", type: "string" },
            { name: "version", type: "string" },
            { name: "chainId", type: "uint256" },
          ],
          Acceptance: [{ name: "content", type: "string" }],
        },
      };

      if (provider.isWalletConnect) {
        // Provider.signer means we have walletConnect
        const sessions = provider.signer.client.session.getAll();
        const session = sessions[sessions.length - 1];

        return await provider.signer.client.request({
          topic: session.topic,
          chainId: `eip155:${Settings.rsk.chainId}`,
          request: {
            method: "eth_signTypedData_v4",
            params: [address, JSON.stringify(typedData)],
          },
        });
      } else {
        return await provider.request({
          method: "eth_signTypedData_v4",
          params: [address, JSON.stringify(typedData)],
          from: address,
        });
      }
    } catch (e) {
      throw new HttpError("Unauthorized", 401, {
        message: e.toString(),
      });
    }
  };

  return (
    <ContractsContext.Provider value={{ contracts, signLoginMessage }}>
      {children}
    </ContractsContext.Provider>
  );
};

export const useContracts = () => useContext(ContractsContext);
