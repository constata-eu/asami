const development = {
  apiDomain: "http://127.0.0.1:8000",
  environment: "development",
  recaptchaSiteKey: "6LeEosgoAAAAAEvZM2fmutlMhYwFMtpFwo_3BIDX",
  rsk: {
    chainId: 31337,
    rpcUrls: {
      31337: "http://localhost:8545",
    },
    supportedChains: [
      {
        chainId: "0x7a69", // hex 31337
        chainName: "RSK Local",
        nativeCurrency: {
          name: "Local RSK BTC",
          symbol: "lRBTC",
          decimals: 18,
        },
        rpcUrls: ["http://localhost:8545"],
        blockExplorerUrls: ["http://localhost:8545"],
        iconUrls: [
          "https://developers.rsk.co/assets/img/favicons/android-chrome-192x192.png",
        ],
      },
    ],
  },
  x: {
    clientId: "REVmR09KdlhzNzVPcHpId2tVUWc6MTpjaQ",
    redirectUri: "http://127.0.0.1:8000",
  },
};

const staging = {
  apiDomain: "https://asami-staging.constata.eu",
  environment: "staging",
  recaptchaSiteKey: "6LeEosgoAAAAAEvZM2fmutlMhYwFMtpFwo_3BIDX",
  rsk: {
    chainId: 31,
    rpcUrls: {
      31: "https://public-node.testnet.rsk.co",
    },
    supportedChains: [
      {
        chainId: "0x1f", // hex 31
        chainName: "RSK Testnet",
        nativeCurrency: {
          name: "TEST RSK BTC",
          symbol: "tRBTC",
          decimals: 18,
        },
        rpcUrls: ["https://public-node.testnet.rsk.co"],
        blockExplorerUrls: ["https://explorer.testnet.rsk.co"],
        iconUrls: [
          "https://developers.rsk.co/assets/img/favicons/android-chrome-192x192.png",
        ],
      },
    ],
  },
  x: {
    clientId: "ZDJUaWk3OVpEU3JTaW12VFFBQmg6MTpjaQ",
    redirectUri: "https://asami-staging.constata.eu",
  },
};

const production = {
  apiDomain: "https://api.asami.club",
  environment: "production",
  recaptchaSiteKey: "6LeEosgoAAAAAEvZM2fmutlMhYwFMtpFwo_3BIDX",
  rsk: {
    chainId: 30,
    rpcUrls: {
      30: "https://public-node.rsk.co",
    },
    supportedChains: [
      {
        chainId: "0x1e", // hex 30
        chainName: "RSK Mainnet",
        nativeCurrency: {
          name: "RSK BTC",
          symbol: "RBTC",
          decimals: 18,
        },
        rpcUrls: ["https://public-node.rsk.co"],
        blockExplorerUrls: ["https://explorer.rsk.co"],
        iconUrls: [
          "https://developers.rsk.co/assets/img/favicons/android-chrome-192x192.png",
        ],
      },
    ],
  },
  x: {
    clientId: "ZDJUaWk3OVpEU3JTaW12VFFBQmg6MTpjaQ",
    redirectUri: "https://api.asami.club",
  },
};

const all = {
  "http://192.168.1.118:5173": production,
  "https://8453.asami.club": production,
  "http://127.0.0.1:5173": development,
  "http://127.0.0.1:4173": development,
  "http://localhost:5173": development,
  "http://localhost:4173": development,
  "https://staging.asami.club": staging,
  "https://asami.club": production,
};

export const Settings = all[window.origin];
