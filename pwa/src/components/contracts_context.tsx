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

  const contracts = async () => {
    if (values) {
      return values;
    }

    const config = (await (await fetch(`${Settings.apiDomain}/config`)).json());
    const {provider, disconnect} = await rLogin.connect();
    const ethersProvider = new ethers.BrowserProvider(provider);
    // ToDo: How should we use provider.disconnect?
    
    const signer = await ethersProvider.getSigner(0);
    const asamiAddress = config.contractAddress;
    const docAddress = config.docContractAddress;
    const asami = new ethers.Contract(asamiAddress, asamiABI.abi, signer);
    const doc = new ethers.Contract(docAddress, docABI, signer);
    const newVals = {doc, asami, asamiAddress, docAddress, signer, provider};
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

      // Listen to the events emitted by the wallet. If changing account, remove the listeners
      // below and connect again. If disconnect or change chains, then logout.
      // Trigger logout
      //    ethersProvider.on('accountsChanged', (accounts) => {
      //      
      //      if (accounts.length === 0) {
      //        return handleLogOut(response)
      //      }
      //      provider.removeAllListeners && provider.removeAllListeners()
      //      handleLogin()
      //    })
      //    provider.on('chainChanged', () => handleLogOut(response))
      //    provider.on('disconnect', () => handleLogOut(response))

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


/*

/////////////
- When the user connects their wallet they get a full web3 context and access to the contracts.

- Having access to the contracts provider does not mean the user is web3 yet. 

- The contracts provider is loaded and memoized on demand by the claim account button, the web3 login, and the account state.

//////
- AccountContext is another controller, wraps the useShowController for Account.


/// The account state component uses both and depending on the account state it may or may not use the contracts provider to fetch some data from the contract itself.
//  - Balances are not cached.
//

- A user that has just claimed their account will get a *new* session immediately.
  - "You have been promoted to a web3 user, from now on you can only use your wallet to log-in".

- If the claimAccount is still pending, components continue fetching things from the API with the new session.

- When a user tries to log-in with a web2 method after claiming, they're notified to use the new login method.

*/



