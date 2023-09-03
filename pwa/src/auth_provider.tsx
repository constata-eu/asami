import { AuthProvider, HttpError } from "react-admin";
import { ethers } from "ethers";

const login = async () => {
  if (typeof window.ethereum === 'undefined') {
    throw (new HttpError("Unauthorized", 401, {
      message: "You need to have metamask or equivalent wallet installed",
    }));
  }
  
  let provider;
  let signer;
  try {
    provider = new ethers.BrowserProvider(window.ethereum);
    signer = await provider.getSigner();
    await provider.send("eth_requestAccounts", []);
  } catch(e) {
    throw (new HttpError("Unauthorized", 401, {
      message: "Cannot log-in if you don't authorize the app.",
    }));
  }

  const user = {
    id: signer.address,
    fullName: `${signer.address.substring(0,6)}…${signer.address.substring(38)}`
  };

  localStorage.setItem("user", JSON.stringify(user));
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
    try {
      await login();
    } catch(e) {
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
