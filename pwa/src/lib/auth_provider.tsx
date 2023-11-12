/* The auth_provider knows about the current login as reported to the react app
 * before attempting to use the data provider.
 * It can create, retrieve and delete the private key from local storage.
 * It uses a special data provider for registering a new public key to the server together with authentication details.
 */

import { AuthProvider, HttpError } from "react-admin";
import { ethers } from "ethers";
import { rLogin } from "./rLogin";
import * as React from 'react';
import { Settings } from '../settings';
import _ from 'lodash';
import { createSessionDataProvider, defaultDataProvider, sha256sum} from "./data_provider";
import {generateKeyPair, exportSPKI, exportPKCS8 } from 'jose';
import pkceChallenge from "pkce-challenge";

export function getAuthKeys() {
  return {
    "sessionPublicKey": localStorage.getItem("sessionPublicKey"),
    "sessionPrivateKey": localStorage.getItem("sessionPrivateKey"),
    "session": JSON.parse(localStorage.getItem("session")),
  };
}

export function setAuthKeys(object, session) {
  if (!_.every(['sessionPublicKey', 'sessionPrivateKey'], (k) => object[k])){
    return false;
  }
  localStorage.setItem("sessionPublicKey", object.sessionPublicKey);
  localStorage.setItem("sessionPrivateKey", object.sessionPrivateKey);
  localStorage.setItem("session", JSON.stringify(session));
  return true;
}

function clearAuthKeys() {
  for(const k of ['sessionPublicKey', 'sessionPrivateKey', 'session']) {
    localStorage.removeItem(k);
  }
}

export function makeInstagramUrl() {
  const { clientId, redirectUri } = Settings.instagram;
  return `https://api.instagram.com/oauth/authorize?client_id=${clientId}&redirect_uri=${redirectUri}&scope=user_profile,user_media&response_type=code`;
}

export async function makeXUrl() {
  const { clientId, redirectUri } = Settings.x;

  let code_verifier = generateRandomString();
  let code_challenge = await pkceChallengeFromVerifier(code_verifier);

  const state = "X-login";
  return {
    "url": `https://x.com/i/oauth2/authorize?response_type=code&client_id=${clientId}&redirect_uri=${redirectUri}&scope=tweet.read%20users.read%20follows.read&state=${state}&code_challenge=${code_challenge}&code_challenge_method=S256`,
    "verifier": code_verifier
  };
}

// Generate a secure random string using the browser crypto functions
function generateRandomString() {
    var array = new Uint32Array(28);
    window.crypto.getRandomValues(array);
    return Array.from(array, dec => ('0' + dec.toString(16)).substr(-2)).join('');
}

// Calculate the SHA256 hash of the input text. 
// Returns a promise that resolves to an ArrayBuffer
function sha256(plain) {
    const encoder = new TextEncoder();
    const data = encoder.encode(plain);
    return window.crypto.subtle.digest('SHA-256', data);
}

// Base64-urlencodes the input string
function base64urlencode(str) {
    // Convert the ArrayBuffer to string using Uint8 array to conver to what btoa accepts.
    // btoa accepts chars only within ascii 0-255 and base64 encodes them.
    // Then convert the base64 encoded to base64url encoded
    //   (replace + with -, replace / with _, trim trailing =)
    return btoa(String.fromCharCode.apply(null, new Uint8Array(str)))
        .replace(/\+/g, '-').replace(/\//g, '_').replace(/=+$/, '');
}

// Return the base64-urlencoded sha256 hash for the PKCE challenge
async function pkceChallengeFromVerifier(v) {
    const hashed = await sha256(v);
    return base64urlencode(hashed);
}

export const makeKeys = async () => {
  const { publicKey, privateKey } = await generateKeyPair('ES256', { extractable: true });
  return {
    sessionPublicKey: (await exportSPKI(publicKey)),
    sessionPrivateKey: (await exportPKCS8(privateKey)),
  };
}

export const rLoginStart = async () => {
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
          // Defining the chain aka Rinkeby testnet or Ethereum Main Net
          chainId: Settings.rsk.chainId,
          // Give a user friendly name to the specific contract you are signing for.
          name: 'Asami',
          // Just let's you know the latest version. Definitely make sure the field name is correct.
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

export const login = async (authMethodKind, authData, recaptchaToken) => {
  try {
    const keys = await makeKeys();
    const dataProvider = await createSessionDataProvider(keys, authMethodKind, authData, recaptchaToken);
    const { data } = await dataProvider.create('Session', { data: {} });
    setAuthKeys(keys, data);
    return Promise.resolve();
  } catch (e) {
    return Promise.reject(e);
  }
};

export const authProvider: AuthProvider = {
  login,
  checkError: (error, graphQLErrors) => {
    const graphQLFail = graphQLErrors?.[0].message === "401";
    const status = error.status || error?.networkError?.statusCode;
    const httpFail = status === 401 || status === 403

    if (graphQLFail) { return Promise.reject(graphQLErrors); }
    if (httpFail) { return Promise.reject(error); }

    return Promise.resolve();
  },
  checkAuth: () => {
    let allSet = _.every(
      ['sessionPublicKey', 'sessionPrivateKey'],
      (k) => localStorage.getItem(k)
    );
    return allSet ? Promise.resolve() : Promise.reject( { redirectTo: '/login' } )
  },
  logout: () => {
    clearAuthKeys();
    return Promise.resolve();
  },
  getIdentity: () => {
    const storedSession = localStorage.getItem("session");
    const session = storedSession ? JSON.parse(storedSession) : null;
    return Promise.resolve({"id": session.id, "fullName": `Member #${session.user_id}`, "avatar": null});
  },
  getPermissions: () => Promise.resolve(),
};

export default authProvider;
