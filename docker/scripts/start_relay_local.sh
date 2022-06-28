#!/bin/sh -xe

SEED_PREFIX=Uf2IucQ3Fgm86
NODE_KEY=`echo "seed ${SEED_PREFIX}//relay//${ACCOUNT}" | sha256sum | sed 's,^.,0,;s, *-,,'`
BASE_PATH=/rio/keys/relay-`printf "%02d" ${ACCOUNT}`

/rio/release/relaychain-rio \
    --base-path ${BASE_PATH} \
    --validator \
    --chain ${RELAY_RAW} \
    --node-key ${NODE_KEY} \
    --unsafe-ws-external \
    --unsafe-rpc-external \
    --no-telemetry \
    --no-prometheus \
    --rpc-cors all \
    --rpc-methods Unsafe \
    --name ${ACCOUNT} \
    --ws-port ${WS_PORT} \
    --rpc-port ${RPC_PORT}
