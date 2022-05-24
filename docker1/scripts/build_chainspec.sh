#!/bin/sh -ex

ls
mkdir -p /tmp/docker1/.chainspec

#rm /tmp/docker1/.chainspec/*

#ls -la /tmp/
#exit 1

./relaychain-rio build-spec --chain polkadot-local --disable-default-bootnode > /tmp/docker1/.chainspec/polkadot-custom-2-plain.json
#./relaychain-rio build-spec --chain rococo-local --disable-default-bootnode > /tmp/docker1/.chainspec/polkadot-custom-2-plain.json
#./relaychain-rio build-spec --chain westend-local --disable-default-bootnode > /tmp/docker1/.chainspec/polkadot-custom-2-plain.json
./parachain-rio build-spec --disable-default-bootnode > /tmp/docker1/.chainspec/parachain-rio-custom-plain.json
#./parachain-rio build-spec --chain statemint-local --disable-default-bootnode > /tmp/docker1/.chainspec/parachain-rio-custom-plain.json
#./polkadot build-spec --chain polkadot-staging --disable-default-bootnode > /tmp/docker1/.chainspec/polkadot-custom-2-plain.json
#./polkadot build-spec --chain /tmp/docker1/.chainspec/polkadot-custom-2-plain.json --raw --disable-default-bootnode > /tmp/docker1/.chainspec/polkadot-custom-2-raw.json

nix-env -i nodejs

export FILE_FROM=/tmp/docker1/.chainspec/polkadot-custom-2-plain.json
export PARA_FILE_FROM=/tmp/docker1/.chainspec/parachain-rio-custom-plain.json
export FILE_TO=/tmp/docker1/.chainspec/1.json
export PARA_FILE_TO=/tmp/docker1/.chainspec/2.json
cd /tmp/docker1/chainspec && npm i
rm -f /tmp/docker1/.chainspec/1.json
rm -f /tmp/docker1/.chainspec/2.json
node /tmp/docker1/chainspec/para_index.js

cd /tmp/riochain
./target/release/parachain-rio build-spec --chain /tmp/docker1/.chainspec/2.json --raw --disable-default-bootnode > /tmp/docker1/.chainspec/parachain-rio-custom-raw.json

./target/release/parachain-rio export-genesis-wasm --chain /tmp/docker1/.chainspec/parachain-rio-custom-raw.json > /tmp/para-2000-wasm
./target/release/parachain-rio export-genesis-state --chain /tmp/docker1/.chainspec/parachain-rio-custom-raw.json > /tmp/para-2000-genesis

node /tmp/docker1/chainspec/index.js

#cp /tmp/docker1/.chainspec/1.json westend-custom.json
cp /tmp/docker1/.chainspec/1.json polkadot-custom.json
#cp /tmp/docker1/.chainspec/1.json rococo-custom.json
#./target/release/relaychain-rio build-spec --chain rococo-custom.json --raw --disable-default-bootnode > /tmp/docker1/.chainspec/rococo-custom-2-raw.json
./target/release/relaychain-rio build-spec --chain polkadot-custom.json --raw --disable-default-bootnode > /tmp/docker1/.chainspec/polkadot-custom-2-raw.json
#./target/release/relaychain-rio build-spec --chain westend-custom.json --raw --disable-default-bootnode > /tmp/docker1/.chainspec/polkadot-custom-2-raw.json

#cp /tmp/para-2000-* /debug/

rm -rf /tmp/node*
node /tmp/docker1/chainspec/insert_keys.js

if [ ! -p /tmp/chain_spec.wakeup ]; then
  mkfifo /tmp/chain_spec.wakeup
fi

set +x
while true; do
  echo ready > /tmp/chain_spec.wakeup
  sleep 1
done




