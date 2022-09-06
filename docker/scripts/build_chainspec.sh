#!/bin/sh -ex

if [ "$MNEMONIC" = "" ]; then
    echo "MNEMONIC is not set: '$MNEMONIC'"
    exit 1
fi

mkdir -p /rio/chainspec || true

cd /rio/src/docker/chainspec && npm i
cd /rio/chainspec

if [ -f "/rio/src/docker/chainspec/airdrop.js" ]; then
    node /rio/src/docker/chainspec/airdrop.js --action=accounts
    node /rio/src/docker/chainspec/airdrop.js --action=balances
fi

/rio/src/target/release/relaychain-rio build-spec --chain polkadot-local --disable-default-bootnode > $RELAY_FILE_FROM
/rio/src/target/release/parachain-rio build-spec --disable-default-bootnode > $PARA_FILE_FROM

# generate para-chainspec
rm -f $PARA_FILE_TO
node /rio/src/docker/chainspec/para_index.js
node /rio/src/docker/chainspec/airdrop.js --action=export --chainspec-parachain=$PARA_FILE_TO

/rio/src/target/release/parachain-rio build-spec --chain $PARA_FILE_TO --raw --disable-default-bootnode > $PARA_RAW

# generate relay-chainspec
rm -f $RELAY_FILE_TO
/rio/src/target/release/parachain-rio export-genesis-wasm --chain $PARA_RAW > $PARA_WASM
/rio/src/target/release/parachain-rio export-genesis-state --chain $PARA_RAW > $PARA_GENESIS

node /rio/src/docker/chainspec/index.js
node /rio/src/docker/chainspec/airdrop.js --action=export --chainspec-polkadot=$RELAY_FILE_TO

/rio/src/target/release/relaychain-rio build-spec --chain $RELAY_FILE_TO --raw --disable-default-bootnode > $RELAY_RAW

# generating keys
rm -rf /rio/keys/*
#mkdir /rio/keys
node /rio/src/docker/chainspec/insert_keys.js



