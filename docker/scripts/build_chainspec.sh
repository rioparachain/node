#!/bin/sh -ex

mkdir -p /rio/chainspec || true

./target/release/relaychain-rio build-spec --chain polkadot-local --disable-default-bootnode > $RELAY_FILE_FROM
./target/release/parachain-rio build-spec --disable-default-bootnode > $PARA_FILE_FROM

cd /rio/src/docker/chainspec && npm i
rm -f $RELAY_FILE_TO
rm -f $PARA_FILE_TO
node /rio/src/docker/chainspec/para_index.js

cd /rio/src

./target/release/parachain-rio build-spec --chain $PARA_FILE_TO --raw --disable-default-bootnode > $PARA_RAW

./target/release/parachain-rio export-genesis-wasm --chain $PARA_RAW > $PARA_WASM
./target/release/parachain-rio export-genesis-state --chain $PARA_RAW > $PARA_GENESIS

node /rio/src/docker/chainspec/index.js

./target/release/relaychain-rio build-spec --chain $RELAY_FILE_TO --raw --disable-default-bootnode > $RELAY_RAW

rm -rf /rio/keys/*
#mkdir /rio/keys
node /rio/src/docker/chainspec/insert_keys.js



