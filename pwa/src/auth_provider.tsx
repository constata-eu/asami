import { AuthProvider, HttpError } from "react-admin";
import { ethers } from "ethers";
import { rLogin } from "./lib/rLogin";

/*
 * ToDo: Manage all possible login types, including EIP712
 */

const login = async () => {
  try {
    const {provider, disconnect } = await rLogin.connect();
    const ethersProvider = new ethers.BrowserProvider(provider);
    const signer = await ethersProvider.getSigner(0);

    const user = {
      id: signer.address,
      fullName: `${signer.address.substring(0,6)}…${signer.address.substring(38)}`
    };

    // Listen to the events emitted by the wallet. If changing account, remove the listeners
    // below and connect again. If disconnect or change chains, then logout.
    // Trigger logout
    /*
        ethersProvider.on('accountsChanged', (accounts) => {
          
          if (accounts.length === 0) {
            return handleLogOut(response)
          }
          provider.removeAllListeners && provider.removeAllListeners()
          handleLogin()
        })
        provider.on('chainChanged', () => handleLogOut(response))
        provider.on('disconnect', () => handleLogOut(response))
    */

    let msgParams = {
        domain: {
          // Defining the chain aka Rinkeby testnet or Ethereum Main Net
          chainId: chainId,
          // Give a user friendly name to the specific contract you are signing for.
          name: 'Ether Mail',
          // Just let's you know the latest version. Definitely make sure the field name is correct.
          version: '1',
        },
        message: {
          contents: 'Login to Asami',
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



    return localStorage.setItem("user", JSON.stringify(user));

  } catch(e) {
    throw (new HttpError("Unauthorized", 401, {
      message: "Cannot log-in if you don't authorize the app.",
    }));
  }
}

export const authProvider: AuthProvider = {
  login,
  logout: () => {
    localStorage.removeItem("user");
    return Promise.resolve();
  },
  checkError: () => Promise.resolve(),
  checkAuth: async () => {
    const text = "Please login to continue";
    if (!localStorage.getItem("user")) {
      throw text;
    }
  },
  getPermissions: () => {
    return Promise.resolve(undefined);
  },
  getIdentity: () => {
    const persistedUser = localStorage.getItem("user");
    const user = persistedUser ? JSON.parse(persistedUser) : null;
    return Promise.resolve(user);
  },
};

export default authProvider;
