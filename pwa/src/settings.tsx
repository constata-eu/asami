const development = {
  apiDomain: "http://127.0.0.1:8000",
  environment: 'development',
  recaptchaSiteKey: "6LeEosgoAAAAAEvZM2fmutlMhYwFMtpFwo_3BIDX",
  rsk: {
    chainId: 1337,
  },
  x: {
    clientId: "ZDJUaWk3OVpEU3JTaW12VFFBQmg6MTpjaQ",
    redirectUri: "http://127.0.0.1:8000/x_login",
  },
  facebook: {
    appId: "376919484899990",
    redirectUri: "http://localhost:8000/facebook_login",
  }
};

const staging = {
  apiDomain: "https://asami-staging.constata.eu",
  environment: 'staging',
  recaptchaSiteKey: "6LeEosgoAAAAAEvZM2fmutlMhYwFMtpFwo_3BIDX",
  rsk: {
    chainId: 31,
  },
  x: {
    clientId: "ZDJUaWk3OVpEU3JTaW12VFFBQmg6MTpjaQ",
    redirectUri: "https://asami-staging.constata.eu/x_login",
  },
  facebook: {
    appId: "376919484899990",
    redirectUri: "https://asami-staging.constata.eu/facebook_login",
  }
};

const production = {
  apiDomain: "https://asami.constata.eu",
  environment: 'production',
  recaptchaSiteKey: "6LeEosgoAAAAAEvZM2fmutlMhYwFMtpFwo_3BIDX",
  rsk: {
    chainId: 30,
  },
  x: {
    clientId: "ZDJUaWk3OVpEU3JTaW12VFFBQmg6MTpjaQ",
    redirectUri: "https://asami.constata.eu/x_login",
  },
  facebook: {
    appId: "376919484899990",
    redirectUri: "https://asami.constata.eu/facebook_login",
  }
};

const all = {
  "http://127.0.0.1:5173": development,
  "http://127.0.0.1:4173": development,
  "http://localhost:5173": development,
  "http://localhost:4173": development,
  "https://staging.asami.club": staging,
  "https://asami.club": production,
}

export const Settings = all[window.origin];
