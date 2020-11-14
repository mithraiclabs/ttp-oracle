# Solana TTP Oracle

[Motivation/Roadmap](https://docs.google.com/document/d/1JfR-6JKM-GXqPkLPdKuKM3vQIjuBVjT2SLMb5MkGLz8/edit?usp=sharing)

## Solana Hackathon Project Description
Mithraic Oracle is the basis of a “built for Solana” oracle. The project is starting out as a simple Trusted Third Party (TTP) oracle. In the base form of the protocol, a Solana program developer can easily make various HTTP(S) requests and receive responses from a trusted server as a callback to their program. The TTP architecture was chosen for its simplicity due to the short time span of the hackathon. While the protocol is simple in its infancy, it’s built in such a way to be the foundation of a more ambitious decentralized oracle system built specifically for the Solana ecosystem. We’re excited about this project because it unlocks more applications on top of Solana such as a CLOB prediction market built on Serum.

## Getting Started

### Install Solana 1.4.4 tool suite 
see https://docs.solana.com/cli/install-solana-cli-tools
```
sh -c "$(curl -sSfL https://release.solana.com/v1.4.4/install)"
```

### Install deps
```
yarn install
```

### Build all Solana programs
```
yarn build
```

### Run local net
```
yarn localnet:up
```

### Deploy programs
```
yarn deploy:local
```

### Run Oracle server in dev environment
This will export all necessary environment variables, build, and run the oracle server.
```
yarn setup-server
```

### Send Sample requests using the Example Client Program
*ORACLE_ID will be generated during the server setup. Copy/paste it as the first argument*

*--loop (or any second argument) can be used to send requests at a .5s interval*

```
yarn send-test [ORACLE_ID] [--loop]
```

### Listen to program logs to see incoming price data
```
docker logs solana-localnet -f | grep -E "Program log"
```

## Testing

### Run JS integration tests
```
yarn test
```

### Run rust unit tests
```
yarn test:programs [PROGRAM_NAME]
```