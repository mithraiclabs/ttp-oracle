### TTP Oracle Server

## Setup

Add the following environment variables

```
ORACLE_ID // optional base58 encoded string, if not provided one will be created on start

ORACLE_PROGRAM_ID // Pubkey string
SOLANA_PRIVATE_KEY // utf-8 string
```

## How to run

### Generate a keypair & store private key as env var

```
solana-keygen new [--outfile KEYPAIR_PATH]
```

```
export SOLANA_PRIVATE_KEY=$(cat [KEYPAIR_PATH])
```

```
export SOLANA_PUBKEY=`solana-keygen pubkey [KEYPAIR_PATH]`
```

### Get funds (airdrop on test nets)

Note this must be done every time the network restarts

```
solana airdrop [AMOUNT] $SOLANA_PUBKEY [--url ENV_URL]
```

### Create Oracle Data Account

```
yarn generate-oracle-account
```

### Build and Run

`yarn build`
`yarn start`
