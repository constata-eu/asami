const development = {
  apiDomain: "http://127.0.0.1:8000",
  environment: 'development',
  recaptchaSiteKey: "6LcMyXYoAAAAABLRzxeVZ1nae06LVvyv8l3O0LSz",
  rsk: {
    chainId: 31,
  },
  x: {
    clientId: "ZDJUaWk3OVpEU3JTaW12VFFBQmg6MTpjaQ",
    redirectUri: "http://127.0.0.1:8000/x_login",
  },
  instagram: {
    clientId: "3539139656337829",
    redirectUri: "https://127.0.0.1:8000/instagram_login",
  }
};

const all = {
  "http://127.0.0.1:5173": development,
  "http://localhost:5173": development,
}

export const Settings = all[window.origin];