#!/bin/sh -xe

set +x
while [ ! -p /tmp/chain_spec.wakeup ]; do
  sleep 1
done
set -x
cat /tmp/chain_spec.wakeup

cd /tmp/riochain

#export RUST_LOG="runtime=debug,executor=debug,parachain::pvf=trace"
#export RUST_LOG=debug

./target/release/parachain-rio \
  --base-path ${BASE_PATH} \
  --collator \
  --chain ${PARA_CHAINSPEC_RAW} \
  --node-key ${NODE_KEY} \
  --unsafe-ws-external \
  --unsafe-rpc-external \
  --no-telemetry \
  --no-prometheus \
  --rpc-cors all \
  --rpc-methods Unsafe \
  --name $ACCOUNT \
  --force-authoring \
  -- \
  --execution wasm \
  --chain ${CHAINSPEC_RAW}
  #2>&1 | grep -E -v '(wasmtime_crane|libp2p_ping|wasm.heap|wasm_overrides|netlink_proto|tokio-runtime-worker afg)'
		       
#  --execution wasm \
#--force-authoring \

