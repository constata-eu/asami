import asamiABI from "../abi/Asami.json";
import docABI from "../abi/Doc.json";
import { rLogin } from "../lib/rLogin";
import { ethers } from "ethers";
import { useDataProvider, useSafeSetState} from "react-admin";
import React, { createContext, useContext } from 'react';
import { HttpError } from "react-admin";
import { Settings } from '../settings';

const ContractsContext = createContext(null);

export const ContractsProvider = ({ children }) => {
  const [values, setValues] = useSafeSetState(null);

  const require_signer = async (expected, actual, disconnect) => {
    if (expected && expected.toLowerCase() != actual.toLowerCase()) {
      await disconnect;
      throw({code: "WRONG_SIGNER", expected, actual});
    }
  }

  const contracts = async (expected_signer) => {
    if (values) {
      await require_signer(expected_signer, values.signer.address, values.disconnect);
      return values;
    }

    const config = (await (await fetch(`${Settings.apiDomain}/config`)).json());
    const {provider, disconnect} = await rLogin.connect();
    provider.on('accountsChanged', async () => { await disconnect; setValues(null); });
    provider.on('disconnect', () => { setValues(null); });
    const ethersProvider = new ethers.BrowserProvider(provider);
    
    const signer = await ethersProvider.getSigner(0);
    await require_signer(expected_signer, signer.address, disconnect);

    const asamiAddress = config.contractAddress;
    const docAddress = config.docContractAddress;
    const asami = new ethers.Contract(asamiAddress, asamiABI.abi, signer);
    const doc = new ethers.Contract(docAddress, docABI, signer);
    const newVals = {doc, asami, asamiAddress, docAddress, signer, provider, disconnect};
    setValues(newVals);
    return newVals;
  }

  const signLoginMessage = async () => {
    try {
      const { signer, provider } = await contracts();

      const user = {
        id: signer.address,
        fullName: `${signer.address.substring(0,6)}…${signer.address.substring(38)}`
      };

      let msgParams = {
          domain: {
            chainId: Settings.rsk.chainId,
            name: 'Asami',
            version: '1',
          },
          message: {
            content: 'Login to Asami',
          },
          primaryType: 'Acceptance',
          types: {
            EIP712Domain: [
              { name: 'name', type: 'string' },
              { name: 'version', type: 'string' },
              { name: 'chainId', type: 'uint256' },
            ],
            Acceptance: [
              { name: 'content', type: 'string' }
            ],
          },
      };

      return await provider.request({
        method: 'eth_signTypedData_v4',
        params: [ signer.address, JSON.stringify(msgParams) ],
        from: signer.address
      });
    } catch(e) {
      throw (new HttpError("Unauthorized", 401, {
        message: "Cannot log-in if you don't authorize the app.",
      }));
    }
  }

  return (
    <ContractsContext.Provider value={{ contracts, signLoginMessage }}>
      {children}
    </ContractsContext.Provider>
  );
};

export const useContracts = () => useContext(ContractsContext);
