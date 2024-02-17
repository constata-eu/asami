import { HttpError } from 'react-admin';
import type { Credentials } from './types';
import { ApolloClient, InMemoryCache, HttpLink, from } from '@apollo/client';
import buildGraphQLProvider, { buildQuery } from 'ra-data-graphql-simple';
import gql from 'graphql-tag';
import { getAuthKeys } from "./auth_provider";
import {SignJWT, importPKCS8} from 'jose';
import { Buffer } from 'buffer';

import { Settings } from '../settings';
export const apiUrl = `${Settings.apiDomain || ''}/graphql/`

const schema = async () => {
  return {"schema": (await (await fetch(`${apiUrl}introspect`)).json()).__schema }
}

export const publicDataProvider = async () => {
  const httpLink = new HttpLink({ uri: apiUrl, });
  const client = new ApolloClient({ link: from([ httpLink ]), cache: new InMemoryCache(), });
  const introspection = await schema();
  return (await buildGraphQLProvider({ client, introspection }));
}

export const defaultDataProvider = async () => {
  const httpLink = new HttpLink({
    uri: apiUrl,
    fetch: async (url, req: any, ...more) => {
      return await navigator.locks.request("only_one_request_at_a_time", async () => {
        req.headers["Authentication"] = await makeAuthenticationHeader(getAuthKeys(), url.toString(), req.method, req.body);
        return (await fetch(url, req, ...more));
      })
    }
  });

  const client = new ApolloClient({ link: from([ httpLink ]), cache: new InMemoryCache(), });
  const introspection = await schema();
  return (await buildGraphQLProvider({ client, introspection }));
}

export const createSessionDataProvider = async (keys, authMethodKind, authData, recaptchaToken) => {
  const httpLink = new HttpLink({
    uri: apiUrl,
    fetch: async (url, req: any, ...more) => {
      req.headers["Authentication"] = await makeAuthenticationHeader(keys, url.toString(), req.method, req.body);
      req.headers["Auth-Action"] = "Login";
      req.headers["Auth-Method-Kind"] = authMethodKind;
      req.headers["Auth-Data"] = authData;
      req.headers["New-Session-Recaptcha-Code"] = recaptchaToken;
      req.headers["Login-Pubkey"] = Buffer.from(keys.sessionPublicKey).toString("base64");

      const response = (await fetch(url, req, ...more));

      if (response.status < 200 || response.status >= 300) {
        let json;
        try { json = JSON.parse(text); } catch (e) { }
        return Promise.reject(
          new HttpError(json?.errors[0]?.message || response?.statusText, response.status, json));
      }
      
      return response;
    }
  });

  const client = new ApolloClient({ link: from([ httpLink ]), cache: new InMemoryCache() });
  const introspection = await schema();
  return (await buildGraphQLProvider({ client, introspection }));
}

export const accessTokenDataProvider = async (access_token) => {
  const httpLink = new HttpLink({
    uri: apiUrl,
    fetch: async (url, req: any, ...more) => {
      req.headers["Access-Token"] = access_token;
      return (await fetch(url, req, ...more));
    }
  });

  const client = new ApolloClient({ link: from([ httpLink ]), cache: new InMemoryCache() });
  const webappBuildQuery = introspection => (fetchType, resource, params) => {
    if (resource === 'OneTimeLogin') {
      const parser = function(data){
        return buildQuery(introspection)('GET_ONE', 'Session', params).parseResponse(data);
      }
      return {
        parseResponse: parser,
        variables: params.data,
        query: gql`mutation createOneTimeLogin($input: OneTimeLoginInput!){
          data: createOneTimeLogin(input: $input) {
            id
            nonce
            personId
            orgId
          }
        }`
      };
    } else {
      return buildQuery(introspection)(fetchType, resource, params);
    }
  };

  const introspection = await schema();
  return (await buildGraphQLProvider({ client, buildQuery: webappBuildQuery, introspection }));
};


export async function sha256sum(plaintext){
  return Buffer.from(await crypto.subtle.digest('SHA-256', (new TextEncoder()).encode(plaintext))).toString("hex")
}

export async function makeAuthenticationHeader(conf: Credentials, url: string, method: string, body: string | null) {
  const {pathname, search } = new URL(url, document.location.origin);

  const payload = {
    "path": pathname,
    "method": method,
    "nonce": Date.now().toString(),
    "body_hash": body ? (await sha256sum(body)) : null,
    "query_hash": search.length > 1 ? (await sha256sum(search.substr(1))) : null
  };

  const kid = await sha256sum(conf.sessionPublicKey);
  const key = await importPKCS8(conf.sessionPrivateKey, "ES256")

  const jwt = await new SignJWT(payload)
    .setProtectedHeader({ alg: 'ES256', kid: kid })
    .sign(key);

  return jwt;
}
