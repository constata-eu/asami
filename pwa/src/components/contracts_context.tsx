import asamiABI from "../abi/Asami.json";
import docABI from "../abi/Doc.json";
import { rLogin } from "../lib/rLogin";
import { ethers } from "ethers";
import { useSafeSetState } from "react-admin";
import { createContext, useContext, ReactNode } from "react";
import { HttpError } from "react-admin";
import { Settings } from "../settings";
import { Client, XOConnectProvider } from "xo-connect";

// Define types for context value and provider props
interface ContractsContextType {
  contracts: (expected_signer: string, isEmbedded: boolean) => Promise<any>;
  signLoginMessage: (
    isEmbedded: boolean,
    message?: string,
    expectedSigner?: string | null,
  ) => Promise<any>;
}

interface ContractsProviderProps {
  children: ReactNode; // Define children prop type
}

const ContractsContext = createContext<ContractsContextType | null>(null);

export const ContractsProvider: React.FC<ContractsProviderProps> = ({
  children,
}) => {
  const [values, setValues] = useSafeSetState<any>(null);

  const require_signer = async (
    expected: string,
    actual: string,
    disconnect: () => Promise<void>,
  ) => {
    if (expected && expected.toLowerCase() !== actual.toLowerCase()) {
      await disconnect();
      throw { code: "WRONG_SIGNER", expected, actual };
    }
  };

  const contracts = async (expected_signer: string, isEmbedded: boolean) => {
    if (values) {
      await require_signer(
        expected_signer,
        values.signer.address,
        values.disconnect,
      );
      return values;
    }

    const config = await (await fetch(`${Settings.apiDomain}/config`)).json();

    const connectEmbedded = (): {
      provider: XOConnectProvider;
      disconnect: () => Promise<void>;
    } => {
      const provider = new XOConnectProvider({
        rpcs: {
          "0x1e": "https://go.getblock.io/0f78b6425f48428ba3246eecc55b0f10",
        },
        defaultChainId: "0x1e",
      });

      return {
        provider,
        disconnect: async () => {},
      };
    };

    const { provider, disconnect } = isEmbedded
      ? connectEmbedded()
      : await rLogin.connect();

    provider.on("accountsChanged", async () => {
      await disconnect();
      setValues(null);
    });
    provider.on("disconnect", () => {
      setValues(null);
    });

    const ethersProvider = new ethers.BrowserProvider(provider);

    const signer = await ethersProvider.getSigner(0);

    await require_signer(
      expected_signer,
      await signer.getAddress(),
      disconnect,
    );

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
    message: string = "Login to Asami",
    expectedSigner: string | null = null,
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
