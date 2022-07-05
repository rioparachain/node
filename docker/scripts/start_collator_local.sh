#!/bin/sh -xe

SEED_PREFIX=Uf2IucQ3Fgm86
NODE_KEY=`echo "seed ${SEED_PREFIX}//collator//${ACCOUNT}" | sha256sum | sed 's,^.,0,;s, *-,,'`
BASE_PATH=/rio/keys/collator-`printf "%02d" ${ACCOUNT}`

/rio/release/parachain-rio \
    --base-path ${BASE_PATH} \
    --collator \
    --chain ${PARA_RAW} \
    --node-key ${NODE_KEY} \
    --unsafe-ws-external \
    --unsafe-rpc-external \
    --no-prometheus \
    --rpc-cors all \
    --rpc-methods Unsafe \
    --name collator-${ACCOUNT} \
    --force-authoring \
    --ws-port ${WS_PORT} \
    --rpc-port ${RPC_PORT} \
#    --telemetry-url 'ws://3.89.91.186:8001/submit 0' \
    -- \
    --execution wasm \
    --chain ${RELAY_RAW} \
#    --telemetry-url 'ws://3.89.91.186:8001/submit 0' \
    --name collator-side-${ACCOUNT}

