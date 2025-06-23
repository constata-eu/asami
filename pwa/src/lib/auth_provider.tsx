/* The auth_provider knows about the current login as reported to the react app
 * before attempting to use the data provider.
 * It can create, retrieve and delete the private key from local storage.
 * It uses a special data provider for registering a new public key to the server together with authentication details.
 */

import { AuthProvider } from "react-admin";
import { Settings } from "../settings";
import _ from "lodash";
import { createSessionDataProvider } from "./data_provider";
import { generateKeyPair, exportSPKI, exportPKCS8 } from "jose";

export function getAuthKeys() {
  return {
    sessionPublicKey: localStorage.getItem("sessionPublicKey"),
    sessionPrivateKey: localStorage.getItem("sessionPrivateKey"),
    session: JSON.parse(localStorage.getItem("session")),
  };
}

export function setAuthKeys(object, session) {
  if (!_.every(["sessionPublicKey", "sessionPrivateKey"], (k) => object[k])) {
    return false;
  }
  localStorage.setItem("sessionPublicKey", object.sessionPublicKey);
  localStorage.setItem("sessionPrivateKey", object.sessionPrivateKey);
  localStorage.setItem("session", JSON.stringify(session));
  return true;
}

function clearAuthKeys() {
  for (const k of ["sessionPublicKey", "sessionPrivateKey", "session"]) {
    localStorage.removeItem(k);
  }
}

export async function makeXLogin() {
  return await makeXUrl(
    "tweet.write tweet.read users.read follows.read offline.access",
    "/x_login",
  );
}

export async function makeXAuthorize() {
  return await makeXUrl(
    "tweet.write tweet.read users.read follows.read offline.access",
    "/x_grant_access",
  );
}

export async function makeXUrl(scope: string, uri: string) {
  const { clientId, redirectUri } = Settings.x;

  let verifier = generateRandomString();
  let code_challenge = await pkceChallengeFromVerifier(verifier);

  const baseUrl = "https://x.com/i/oauth2/authorize";
  const params = new URLSearchParams({
    response_type: "code",
    client_id: clientId,
    redirect_uri: redirectUri + uri,
    scope,
    state: "X-login",
    code_challenge,
    code_challenge_method: "S256",
  });

  return { url: `${baseUrl}?${params.toString()}`, verifier };
}

// Generate a secure random string using the browser crypto functions
function generateRandomString() {
  var array = new Uint32Array(28);
  window.crypto.getRandomValues(array);
  return Array.from(array, (dec) => ("0" + dec.toString(16)).substr(-2)).join(
    "",
  );
}

// Calculate the SHA256 hash of the input text.
// Returns a promise that resolves to an ArrayBuffer
function sha256(plain) {
  const encoder = new TextEncoder();
  const data = encoder.encode(plain);
  return window.crypto.subtle.digest("SHA-256", data);
}

// Base64-urlencodes the input string
function base64urlencode(str) {
  // Convert the ArrayBuffer to string using Uint8 array to conver to what btoa accepts.
  // btoa accepts chars only within ascii 0-255 and base64 encodes them.
  // Then convert the base64 encoded to base64url encoded
  //   (replace + with -, replace / with _, trim trailing =)
  return btoa(String.fromCharCode.apply(null, new Uint8Array(str)))
    .replace(/\+/g, "-")
    .replace(/\//g, "_")
    .replace(/=+$/, "");
}

// Return the base64-urlencoded sha256 hash for the PKCE challenge
async function pkceChallengeFromVerifier(v) {
  const hashed = await sha256(v);
  return base64urlencode(hashed);
}

export const makeKeys = async () => {
  const { publicKey, privateKey } = await generateKeyPair("ES256", {
    extractable: true,
  });
  return {
    sessionPublicKey: await exportSPKI(publicKey),
    sessionPrivateKey: await exportPKCS8(privateKey),
  };
};

export const login = async (authMethodKind, authData, recaptchaToken) => {
  try {
    const keys = await makeKeys();
    const dataProvider = await createSessionDataProvider(
      keys,
      authMethodKind,
      authData,
      recaptchaToken,
    );
    const { data } = await dataProvider.create("Session", { data: {} });
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
    const httpFail = status === 401 || status === 403;

    if (graphQLFail) {
      return Promise.reject(graphQLErrors);
    }
    if (httpFail) {
      return Promise.reject(error);
    }

    return Promise.resolve();
  },
  checkAuth: () => {
    let allSet = _.every(["sessionPublicKey", "sessionPrivateKey"], (k) =>
      localStorage.getItem(k),
    );
    return allSet
      ? Promise.resolve()
      : Promise.reject({ redirectTo: "/login" });
  },
  logout: () => {
    clearAuthKeys();
    return Promise.resolve();
  },
  getIdentity: () => {
    const storedSession = localStorage.getItem("session");
    const session = storedSession ? JSON.parse(storedSession) : null;
    return Promise.resolve({
      id: session.id,
      fullName: `Member #${session.user_id}`,
      avatar: null,
    });
  },
  getPermissions: () => Promise.resolve(),
  canAccess: ({ action, resource }) => {
    const storedSession = localStorage.getItem("session");
    const session = storedSession ? JSON.parse(storedSession) : null;

    if (action == "delete") {
      return false;
    }

    if (resource == "Handle" && action == "edit") {
      return session?.admin;
    } else {
      return true;
    }
  },
};

export default authProvider;
