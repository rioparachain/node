#!/bin/sh -xe

#ls -la ${BASE_PATH}/chains/rio_relay_chain_staging_testnet/keystore/


#ls -la ${CHAINSPEC_RAW}

set +x
while [ ! -p /tmp/chain_spec.wakeup ]; do
  sleep 1
done
set -x
cat /tmp/chain_spec.wakeup

#ls -la ${CHAINSPEC_RAW}

#sleep 60

if [ -d /debug ]; then
  cp -Rp /tmp /debug/
fi

#https://docs.substrate.io/tutorials/v3/private-network/
#Enable other participants to join
#You can now allow other validators to join the network using the --bootnodes and --validator command-line options.
#To add a second validator to the private network:
#Open a terminal shell on a second computer.
#Change to the root directory where you compiled the Substrate node template.

cd /tmp/riochain

#if [ "$ACCOUNT" = "1" ]; then
#export RUST_LOG="runtime=debug,executor=debug,parachain::pvf=trace"
#fi

./target/release/relaychain-rio \
  --base-path ${BASE_PATH} \
  --validator \
  --chain ${CHAINSPEC_RAW} \
  --node-key ${NODE_KEY} \
  --unsafe-ws-external \
  --unsafe-rpc-external \
  --no-telemetry \
  --no-prometheus \
  --rpc-cors all \
  --rpc-methods Unsafe \
  --name $ACCOUNT
  # 2>&1 | grep -E -v '(wasmtime_crane|libp2p_ping|wasm.heap|wasm_overrides|netlink_proto|tokio-runtime-worker afg)'

