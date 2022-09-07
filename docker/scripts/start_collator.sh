#!/bin/sh -xe

#export RUST_LOG="runtime=debug,executor=debug,parachain::pvf=trace"
#export RUST_LOG=debug

#find /mnt
#mkdir -p /mnt/${APP_NAME}/${STAGE} || true
#echo '555' >> /mnt/${APP_NAME}/${STAGE}/1.txt

IP_LOCAL=`ip a|grep 'inet '|sed 's/^.*inet //'|sed 's/\/.*$//'|grep -E '1[0-9]\.'`

ACCOUNT=$(curl -s "${DISTRIBUTE_KEYS}/collator")
if [ "$ACCOUNT" = "" ]; then
    sleep 30
    exit 1
fi
ACCOUNT_NAME=`printf "%02d" $ACCOUNT`
BASE_PATH=/rio/keys/collator-`printf "%02d" $ACCOUNT`

# todo - change to more secure random string - get it from AWS secret store
NODE_KEY=`echo "seed ${SEED_PREFIX}//collator//$ACCOUNT" | sha256sum | sed 's,^.,0,;s, *-,,'`
ACCOUNT_PUBLIC_KEY=`echo -n ${NODE_KEY} | /rio/release/parachain-rio key inspect-node-key --file /dev/stdin | tail -n 1`

# update key-distributor
curl -X POST "${DISTRIBUTE_KEYS}/collator/${ACCOUNT}?stage=${STAGE}&ip=${IP_LOCAL}&key=${ACCOUNT_PUBLIC_KEY}"

# fetch nodes state
STATE=$(curl -X GET -s "${DISTRIBUTE_KEYS}")

BOOTNODE_IP=`echo $STATE|jq '.[] | select(.type=="collator" and .account=="1")|.ip'|sed 's/"//g'`
BOOTNODE_KEY=`echo $STATE|jq '.[] | select(.type=="collator" and .account=="1")|.key'|sed 's/"//g'`

RELAY_BOOTNODE_IP=`echo $STATE|jq '.[] | select(.type=="relay" and .account=="1")|.ip'|sed 's/"//g'`
RELAY_BOOTNODE_KEY=`echo $STATE|jq '.[] | select(.type=="relay" and .account=="1")|.key'|sed 's/"//g'`

if [ "$ACCOUNT" = "1" ]; then
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
        --name collator-${ACCOUNT_NAME} \
        --force-authoring \
        --ws-port ${WS_PORT} \
        --rpc-port ${RPC_PORT} \
        --telemetry-url 'ws://3.89.91.186:8001/submit 0' \
        -- \
        --execution wasm \
        --chain ${RELAY_RAW} \
        --telemetry-url 'ws://3.89.91.186:8001/submit 0' \
        --name collator-side-${ACCOUNT_NAME} \
        --bootnodes /ip4/${RELAY_BOOTNODE_IP}/tcp/30333/p2p/${RELAY_BOOTNODE_KEY}
else
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
        --name collator-${ACCOUNT_NAME} \
        --force-authoring \
        --ws-port ${WS_PORT} \
        --rpc-port ${RPC_PORT} \
        --telemetry-url 'ws://3.89.91.186:8001/submit 0' \
        --bootnodes /ip4/${BOOTNODE_IP}/tcp/30333/p2p/${BOOTNODE_KEY} \
        -- \
        --execution wasm \
        --chain ${RELAY_RAW} \
        --telemetry-url 'ws://3.89.91.186:8001/submit 0' \
        --name collator-side-${ACCOUNT_NAME} \
        --bootnodes /ip4/${RELAY_BOOTNODE_IP}/tcp/30333/p2p/${RELAY_BOOTNODE_KEY}
fi
