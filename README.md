# Template for Solana project

## Getting Started

### Install solana tool suite 
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

### Run JS integration tests
```
yarn test
```

### Run rust unit tests
```
yarn test:programs [PROGRAM_NAME]
```

### Creating new program
```
yarn generate-program PROGRAM_NAME
```

### Run Oracle server in dev environment
This will export all necessary environment variables, build, and run the oracle server.
```
yarn setup-server
```
