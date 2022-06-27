#!/bin/sh -xe

NODE_KEY=`echo "seed Uf2IucQ3Fgm86//collator//$ACCOUNT" | sha256sum | sed 's,^.,0,;s, *-,,'`
BASE_PATH=/rio/keys/collator-`printf "%02d" $ACCOUNT`

/rio/release/parachain-rio \
    --base-path ${BASE_PATH} \
    --collator \
    --chain ${PARA_RAW} \
    --node-key ${NODE_KEY} \
    --unsafe-ws-external \
    --unsafe-rpc-external \
    --no-telemetry \
    --no-prometheus \
    --rpc-cors all \
    --rpc-methods Unsafe \
    --name ${ACCOUNT} \
    --force-authoring \
    --ws-port ${WS_PORT} \
    --rpc-port ${RPC_PORT} \
    -- \
    --execution wasm \
    --chain ${RELAY_RAW}
