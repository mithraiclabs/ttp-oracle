#!/usr/bin/env bash

cd "$(dirname "$0")"

solana-keygen new --outfile ../testKey.json

export SOLANA_ENV=dev
export SOLANA_PRIVATE_KEY=$(cat ../testKey.json)

export SOLANA_PUBKEY=`solana-keygen pubkey ../testKey.json`

solana airdrop 100000 $SOLANA_PUBKEY --url http://localhost:8899

oracle_program_id=$(jq '."solana_bpf_ttp_oracle.so"' ../testDeployed.json)
oracle_program_id="${oracle_program_id%\"}"
oracle_program_id="${oracle_program_id#\"}"

export ORACLE_PROGRAM_ID="$oracle_program_id"

output=$(yarn --cwd ../server/ generate-oracle-account)

export ORACLE_ID=$(echo "$output" | sed -n '3 p')

yarn --cwd ../server/ build

yarn --cwd ../server/ start
