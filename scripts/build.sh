#!/usr/bin/env bash

cd "$(dirname "$0")"

for dir in ../programs/*/
do
    program_path=${dir%*/}
    program_name=${program_path##*/}

    cargo build-bpf --manifest-path "${program_path}/Cargo.toml"
done
# clear dist and move all output to it
rm -rf ../dist
mkdir ../dist
find . -name '*.so' -exec mv {} ../dist \;
