import { createConfig, http } from "wagmi";
import { defineChain } from "viem";
import { connectorsForWallets } from "@rainbow-me/rainbowkit";
import {
  injectedWallet,
  rainbowWallet,
  walletConnectWallet,
  metaMaskWallet,
  coinbaseWallet,
} from "@rainbow-me/rainbowkit/wallets";

//  Duckchain mainnet chain
export const duckchainMainnet = defineChain({
  id: 5545,
  name: "DuckChain Mainnet",
  network: "duckchain-mainnet",
  nativeCurrency: {
    decimals: 18,
    name: "TON",
    symbol: "TON",
  },
  rpcUrls: {
    default: {
      http: ["https://rpc.duckchain.io"],
    },
    public: {
      http: ["https://rpc.duckchain.io"],
    },
  },
  blockExplorers: {
    default: {
      name: "Duckscan",
      url: "https://scan.duckchain.io",
    },
  },
  testnet: false,
});

// Lazai testnet
export const lazaiTestnet = defineChain({
  id: 133718,
  name: "Lazai Testnet",
  network: "lazai-testnet",
  nativeCurrency: {
    decimals: 18,
    name: "LAZAI",
    symbol: "LAZAI",
  },
  rpcUrls: {
    default: {
      http: ["https://testnet.lazai.network"],
    },
    public: {
      http: ["https://testnet.lazai.network"],
    },
  },
  blockExplorers: {
    default: {
      name: "Lazai Testnet",
      url: "https://testnet-explorer.lazai.network/",
    },
  },
  testnet: true,
});

// wallet connectors
const connectors = connectorsForWallets(
  [
    {
      groupName: "Recommended",
      wallets: [
        injectedWallet,
        rainbowWallet,
        metaMaskWallet,
        coinbaseWallet,
        walletConnectWallet,
      ],
    },
  ],
  {
    appName: "ENCLAVA",
    projectId: "8d0f880bcadda7b5b3fa580f76de67da",
  }
);

export const config = createConfig({
  chains: [duckchainMainnet, lazaiTestnet],
  connectors,
  transports: {
    [duckchainMainnet.id]: http(),
    [lazaiTestnet.id]: http(),
  },
  ssr: false,
});
