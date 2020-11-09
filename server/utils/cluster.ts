export enum ClusterEnv {
  dev = 'dev',
  devnet = 'devnet',
  testnet = 'testnet',
  mainnetBeta = 'mainnet-beta',
}

type Cluster = {
  rest: Record<ClusterEnv, string>;
  socket: Record<ClusterEnv, string>;
};

export const cluster: Cluster = {
  rest: {
    dev: 'http://localhost:8899',
    devnet: 'https://devnet.solana.com',
    testnet: 'https://testnet.solana.com',
    'mainnet-beta': 'https://api.mainnet-beta.solana.com',
  },
  socket: {
    dev: 'ws://localhost:8899',
    devnet: 'wss://devnet.solana.com',
    testnet: 'wss://testnet.solana.com',
    'mainnet-beta': 'wss://api.mainnet-beta.solana.com',
  },
};
