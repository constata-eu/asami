import RLogin from "@rsksmart/rlogin";
import { WalletConnect2Provider } from "@rsksmart/rlogin-walletconnect2-provider";
import { Settings } from "../settings";

export const rLogin = new RLogin({
  ethereumChains: Settings.rsk.supportedChains,
  providerOptions: {
    walletconnect: {
      package: WalletConnect2Provider,
      options: {
        projectId: "c531d864674ffa6c8d234eaf1ecd3f24",
        chains: [Settings.rsk.chainId],
        showQrModal: true,
        rpcMap: Settings.rsk.rpcUrls,
      },
    },
  },
});
