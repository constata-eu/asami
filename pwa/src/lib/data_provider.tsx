import { HttpError } from "react-admin";
import type { Credentials } from "./types";
import { ApolloClient, InMemoryCache, HttpLink, from } from "@apollo/client";
import buildGraphQLProvider, { buildQuery } from "ra-data-graphql-simple";
import gql from "graphql-tag";
import { getAuthKeys } from "./auth_provider";
import { SignJWT, importPKCS8 } from "jose";
import { Buffer } from "buffer";
import { normalizeRootstockJoinedAt } from "./rootstock_joined_at";

import { Settings } from "../settings";
export const apiUrl = `${Settings.apiDomain || ""}/graphql/`;

const schema = async () => {
  return {
    schema: (await (await fetch(`${apiUrl}introspect`)).json()).__schema,
  };
};

export const publicDataProvider = async () => {
  const httpLink = new HttpLink({ uri: apiUrl });
  const client = new ApolloClient({
    link: from([httpLink]),
    cache: new InMemoryCache(),
  });
  const introspection = await schema();
  return await buildGraphQLProvider({
    client,
    introspection,
    buildQuery: makeBuildQuery(client),
  });
};

export const defaultDataProvider = async () => {
  const httpLink = new HttpLink({
    uri: apiUrl,
    fetch: async (url, req: any, ...more) => {
      return await navigator.locks.request(
        "only_one_request_at_a_time",
        async () => {
          req.headers["Authentication"] = await makeAuthenticationHeader(
            getAuthKeys(),
            url.toString(),
            req.method,
            req.body,
          );
          return await fetch(url, req, ...more);
        },
      );
    },
  });

  const client = new ApolloClient({
    link: from([httpLink]),
    cache: new InMemoryCache(),
  });
  const introspection = await schema();
  return await buildGraphQLProvider({
    client,
    buildQuery: makeBuildQuery(client),
    introspection,
  });
};

const makeBuildQuery =
  (client) => (introspection) => (fetchType, resource, params) => {
    if (resource === "CreateCampaignFromLink") {
      const parser = function (data) {
        return buildQuery(introspection)(
          "GET_ONE",
          "Campaign",
          params,
        ).parseResponse(data);
      };
      return {
        parseResponse: parser,
        variables: params.data,
        query: gql`
          mutation createCampaignFromLink(
            $input: CreateCampaignFromLinkInput!
          ) {
            data: createCampaignFromLink(input: $input) {
              id
              accountId
              budget
              kind
              briefingJson
              briefingHash
              validUntil
              isEnded
              createdAt
              topicIds
              thumbsUpOnly
              privateFields {
                status
              }
            }
          }
        `,
      };
    } else if (resource === "ClaimAccountRequest") {
      const parser = function (data) {
        return buildQuery(introspection)(
          "GET_ONE",
          "Account",
          params,
        ).parseResponse(data);
      };
      return {
        parseResponse: parser,
        variables: params.data,
        query: gql`
          mutation createClaimAccountRequest(
            $input: CreateClaimAccountRequestInput!
          ) {
            data: createClaimAccountRequest(input: $input) {
              id
              status
              addr
              unclaimedAsamiBalance
              unclaimedDocBalance
              asamiBalance
              docBalance
              rbtcBalance
              allowsGasless
            }
          }
        `,
      };
    } else if (resource === "GaslessAllowance") {
      const parser = function (data) {
        return buildQuery(introspection)(
          "GET_ONE",
          "Account",
          params,
        ).parseResponse(data);
      };
      return {
        parseResponse: parser,
        variables: params.data,
        query: gql`
          mutation {
            data: createGaslessAllowance {
              id
              status
              addr
              unclaimedAsamiBalance
              unclaimedDocBalance
              asamiBalance
              docBalance
              rbtcBalance
              allowsGasless
            }
          }
        `,
      };
    } else if (resource === "XRefreshToken") {
      const parser = function (data) {
        return buildQuery(introspection)(
          "GET_ONE",
          "Handle",
          params,
        ).parseResponse(data);
      };
      return {
        parseResponse: parser,
        variables: params.data,
        query: gql`
          mutation ($token: String!, $verifier: String!) {
            data: createXRefreshToken(token: $token, verifier: $verifier) {
              id
              accountId
              username
              userId
              topicIds
              status
              totalCollabs
              totalCollabRewards
            }
          }
        `,
      };
    } else if (resource === "Handle" && fetchType == "UPDATE") {
      const parser = function (data) {
        return buildQuery(introspection)(
          "GET_ONE",
          "Handle",
          params,
        ).parseResponse(data);
      };

      const keys = ["topicIds"];

      let data = {};

      for (let k of keys) {
        data[k] = params.data[k];
      }

      return {
        parseResponse: parser,
        variables: { id: params.data.id, data },
        query: gql`
          mutation ($id: Int!, $data: AdminEditHandleInput!) {
            data: updateHandle(id: $id, data: $data) {
              id
              accountId
              username
              userId
              topicIds
              status
              totalCollabs
              totalCollabRewards
            }
          }
        `,
      };
    } else if (resource === "CommunityMember" && fetchType == "UPDATE") {
      const parser = function (data) {
        return buildQuery(introspection)(
          "GET_ONE",
          "CommunityMember",
          params,
        ).parseResponse(data);
      };

      return {
        parseResponse: parser,
        variables: params,
        query: gql`
          mutation ($id: Int!, $data: EditCommunityMemberInput!) {
            data: updateCommunityMember(id: $id, data: $data) {
              id
              accountId
              memberId
              rating
              collabs
              rewards
              firstCollabDate
              lastCollabDate
            }
          }
        `,
      };
    } else if (resource === "Account" && fetchType == "UPDATE") {
      const parser = function (data) {
        return buildQuery(introspection)(
          "GET_ONE",
          "Account",
          params,
        ).parseResponse(data);
      };

      return {
        parseResponse: parser,
        variables: { id: params.id, data: params.data },
        query: gql`
          mutation ($id: String!, $data: AdminEditAccountInput!) {
            data: updateAccount(id: $id, data: $data) {
              id
              status
              acceptanceStatus
              name
              invitedByAccountId
              invitedByName
              invitesPerMonth
              availableInvites
              alignmentScore
              trustScore
              contributionScore
              advocacyScore
              judgmentScore
              rootstockLongevityScore
              rootstockJoinedAt
              scoresReceivedCount
              scoresIssuedCount
            }
          }
        `,
      };
    } else if (resource === "CommunityMember" && fetchType == "CREATE") {
      const parser = function (data) {
        return buildQuery(introspection)(
          "GET_ONE",
          "CommunityMember",
          params,
        ).parseResponse(data);
      };

      return {
        parseResponse: parser,
        variables: { input: params.data },
        query: gql`
          mutation ($input: CreateCommunityMemberInput!) {
            data: createCommunityMember(input: $input) {
              id
              accountId
              memberId
              priority
              alignment
              trust
              contribution
              advocacy
              judgment
              rootstockJoinedAt
              comment
              collabs
              rewards
              firstCollabDate
              lastCollabDate
            }
          }
        `,
      };
    } else if (
      resource === "CommunityMemberMembership" &&
      fetchType == "CREATE"
    ) {
      const parser = function (data) {
        return buildQuery(introspection)(
          "GET_ONE",
          "CommunityMember",
          params,
        ).parseResponse(data);
      };

      return {
        parseResponse: parser,
        variables: { input: params.data },
        query: gql`
          mutation ($input: CreateCommunityMemberMembershipInput!) {
            data: createCommunityMemberMembership(input: $input) {
              id
              accountId
              memberId
              rating
              priority
              alignment
              trust
              contribution
              advocacy
              judgment
              rootstockJoinedAt
              comment
              collabs
              rewards
              firstCollabDate
              lastCollabDate
            }
          }
        `,
      };
    } else if (resource === "Invitation" && fetchType == "CREATE") {
      const parser = function (data) {
        return { data: { ...data.data, id: data.data.id } };
      };

      return {
        parseResponse: parser,
        variables: { input: params.data },
        query: gql`
          mutation ($input: CreateInvitationInput!) {
            data: createInvitation(input: $input) {
              id
              inviterAccountId
              accountId
              token
              inviteUrl
              createdAt
              expiresAt
              acceptedAt
            }
          }
        `,
      };
    } else if (resource === "WaitlistApplication" && fetchType == "CREATE") {
      return {
        parseResponse: (data) => ({ data: { ...data.data, id: data.data.id } }),
        variables: {
          input: {
            name: params.data.name,
            email: params.data.email,
            link1: params.data.link1,
            link2: params.data.link2,
            link3: params.data.link3,
            note: params.data.note,
          },
        },
        query: gql`
          mutation ($input: CreateWaitlistApplicationInput!) {
            data: createWaitlistApplication(input: $input) {
              id
              name
              email
              link1
              link2
              link3
              note
              status
              reviewNotes
              createdAt
              reviewedAt
            }
          }
        `,
      };
    } else if (resource === "WaitlistApplication" && fetchType == "GET_ONE") {
      return {
        parseResponse: (data) => ({ data: { ...data.data, id: data.data.id } }),
        variables: { id: Number(params.id) },
        query: gql`
          query ($id: Int!) {
            data: WaitlistApplication(id: $id) {
              id
              name
              email
              link1
              link2
              link3
              note
              invitationId
              inviteUrl
              status
              reviewNotes
              createdAt
              reviewedAt
            }
          }
        `,
      };
    } else if (resource === "WaitlistApplication" && fetchType == "UPDATE") {
      const acceptWithInvitation =
        params.data.status === "ACCEPTED"
          ? {
              accountName: params.data.accountName,
              alignment: Number(params.data.alignment),
              trust: Number(params.data.trust),
              contribution: Number(params.data.contribution),
              advocacy: Number(params.data.advocacy),
              judgment: Number(params.data.judgment),
              rootstockJoinedAt: normalizeRootstockJoinedAt(
                params.data.rootstockJoinedAt,
              ),
              comment: params.data.comment || null,
              priority: !!params.data.priority,
            }
          : null;

      return {
        parseResponse: (data) => ({ data: { ...data.data, id: data.data.id } }),
        variables: {
          id: params.id,
          data: {
            status: params.data.status,
            reviewNotes: params.data.reviewNotes || null,
            acceptWithInvitation,
          },
        },
        query: gql`
          mutation ($id: Int!, $data: EditWaitlistApplicationInput!) {
            data: updateWaitlistApplication(id: $id, data: $data) {
              id
              name
              email
              link1
              link2
              link3
              note
              invitationId
              inviteUrl
              status
              reviewNotes
              createdAt
              reviewedAt
            }
          }
        `,
      };
    } else if (resource === "CommunityMemberSkip" && fetchType == "CREATE") {
      const parser = function (data) {
        return { data: { ...data.data, id: data.data.id } };
      };

      return {
        parseResponse: parser,
        variables: { input: params.data },
        query: gql`
          mutation ($input: CreateCommunityMemberSkipInput!) {
            data: createCommunityMemberSkip(input: $input) {
              id
              scorerAccountId
              targetAccountId
              reason
              createdAt
            }
          }
        `,
      };
    } else if (resource === "ScorableAccount" && fetchType == "GET_LIST") {
      return {
        parseResponse: (response) => ({
          data: response.data.data.map((item) => ({
            ...item,
            id: item.accountId,
          })),
          total: response.data.data.length,
        }),
        variables: {},
        query: gql`
          query {
            data: scorableAccounts {
              accountId
              name
              reason
              windowStart
              scorableUntil
            }
          }
        `,
      };
    } else if (resource === "LandingMember" && fetchType == "GET_LIST") {
      return {
        parseResponse: (response) => ({
          data: response.data.data.map((item) => ({
            ...item,
            id: item.accountId,
          })),
          total: response.data.data.length,
        }),
        variables: { limit: params.pagination?.perPage || 6 },
        query: gql`
          query ($limit: Int!) {
            data: topRootstockers(limit: $limit) {
              accountId
              name
              xUsername
              score
              image
            }
          }
        `,
      };
    } else if (resource === "AccountMerge" && fetchType == "UPDATE") {
      const parser = function (data) {
        return { data: { id: params.id } };
      };

      return {
        parseResponse: parser,
        variables: params,
        query: gql`
          mutation ($id: String!) {
            updateAccountMerge(id: $id)
          }
        `,
      };
    } else {
      return buildQuery(introspection)(fetchType, resource, params);
    }
  };

export const createSessionDataProvider = async (
  keys,
  authMethodKind,
  authData,
  recaptchaToken,
) => {
  const httpLink = new HttpLink({
    uri: apiUrl,
    fetch: async (url, req: any, ...more) => {
      req.headers["Authentication"] = await makeAuthenticationHeader(
        keys,
        url.toString(),
        req.method,
        req.body,
      );
      req.headers["Auth-Action"] = "Login";
      req.headers["Auth-Method-Kind"] = authMethodKind;
      req.headers["Auth-Data"] = authData;
      req.headers["New-Session-Recaptcha-Code"] = recaptchaToken;
      req.headers["Login-Pubkey"] = Buffer.from(keys.sessionPublicKey).toString(
        "base64",
      );

      const response = await fetch(url, req, ...more);

      if (response.status < 200 || response.status >= 300) {
        let json = null;
        try {
          json = await response.json();
        } catch {}
        return Promise.reject(
          new HttpError(
            json?.authError || json?.errors[0]?.message || response?.statusText,
            response.status,
            json,
          ),
        );
      }

      return response;
    },
  });

  const client = new ApolloClient({
    link: from([httpLink]),
    cache: new InMemoryCache(),
  });
  const introspection = await schema();
  return await buildGraphQLProvider({ client, introspection });
};

export const accessTokenDataProvider = async (access_token) => {
  const httpLink = new HttpLink({
    uri: apiUrl,
    fetch: async (url, req: any, ...more) => {
      req.headers["Access-Token"] = access_token;
      return await fetch(url, req, ...more);
    },
  });

  const client = new ApolloClient({
    link: from([httpLink]),
    cache: new InMemoryCache(),
  });
  const webappBuildQuery = (introspection) => (fetchType, resource, params) => {
    if (resource === "OneTimeLogin") {
      const parser = function (data) {
        return buildQuery(introspection)(
          "GET_ONE",
          "Session",
          params,
        ).parseResponse(data);
      };
      return {
        parseResponse: parser,
        variables: params.data,
        query: gql`
          mutation createOneTimeLogin($input: OneTimeLoginInput!) {
            data: createOneTimeLogin(input: $input) {
              id
              nonce
              personId
              orgId
            }
          }
        `,
      };
    } else {
      return buildQuery(introspection)(fetchType, resource, params);
    }
  };

  const introspection = await schema();
  return await buildGraphQLProvider({
    client,
    buildQuery: webappBuildQuery,
    introspection,
  });
};

export async function sha256sum(plaintext) {
  return Buffer.from(
    await crypto.subtle.digest("SHA-256", new TextEncoder().encode(plaintext)),
  ).toString("hex");
}

export async function makeAuthenticationHeader(
  conf: Credentials,
  url: string,
  method: string,
  body: string | null,
) {
  const { pathname, search } = new URL(url, document.location.origin);

  const payload = {
    path: pathname,
    method: method,
    nonce: Date.now().toString(),
    body_hash: body ? await sha256sum(body) : null,
    query_hash: search.length > 1 ? await sha256sum(search.substr(1)) : null,
  };

  const kid = await sha256sum(conf.sessionPublicKey);
  const key = await importPKCS8(conf.sessionPrivateKey, "ES256");

  const jwt = await new SignJWT(payload)
    .setProtectedHeader({ alg: "ES256", kid: kid })
    .sign(key);

  return jwt;
}
