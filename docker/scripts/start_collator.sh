#!/bin/sh -xe

#export RUST_LOG="runtime=debug,executor=debug,parachain::pvf=trace"
#export RUST_LOG=debug

find /mnt
mkdir -p /mnt/${APP_NAME}/${STAGE} || true
echo '555' >> /mnt/${APP_NAME}/${STAGE}/1.txt

IP_LOCAL=`ip a | sed 's,[ /], ,g' | awk '/inet 10\./{ print $2 }' | head -n 1`

curl "http://44.202.25.232:3000/collator?stage=${STAGE}" -o cur_account.txt
cat cur_account.txt
ACCOUNT=$(cat cur_account.txt)
if [ "$ACCOUNT" = "" ]; then
    sleep 30
    exit 1
fi
BASE_PATH=/rio/keys/collator-`printf "%02d" $ACCOUNT`

# todo - change to more secure random string - get it from AWS secret store
NODE_KEY=`echo "seed ${SEED_PREFIX}//collator//$ACCOUNT" | sha256sum | sed 's,^.,0,;s, *-,,'`
ACCOUNT_PUBLIC_KEY=`echo -n ${NODE_KEY} | /rio/release/parachain-rio key inspect-node-key --file /dev/stdin | tail -n 1`
curl "http://44.202.25.232:3000/collator/${ACCOUNT}?stage=${STAGE}" -H "content-type: application/json"  -d "{\"key\": \"${ACCOUNT_PUBLIC_KEY}\", \"ip\": \"${IP_LOCAL}\"}" -o bootnodes.json
cat bootnodes.json
BOOTNODE_IP=`node -e "console.log(JSON.parse(require('fs').readFileSync('bootnodes.json'))[0].ip)"`
BOOTNODE_KEY=`node -e "console.log(JSON.parse(require('fs').readFileSync('bootnodes.json'))[0].key)"`

curl "http://44.202.25.232:3000/relay/1?stage=${STAGE}" -o relay_bootnodes.json
cat relay_bootnodes.json
RELAY_BOOTNODE_IP=`node -e "console.log(JSON.parse(require('fs').readFileSync('relay_bootnodes.json'))[0].ip)"`
RELAY_BOOTNODE_KEY=`node -e "console.log(JSON.parse(require('fs').readFileSync('relay_bootnodes.json'))[0].key)"`

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
        --name ${ACCOUNT} \
        --force-authoring \
        --ws-port ${WS_PORT} \
        --rpc-port ${RPC_PORT} \
        --telemetry-url 'ws://3.89.91.186:8001/submit 0' \
        -- \
        --execution wasm \
        --chain ${RELAY_RAW} \
        --telemetry-url 'ws://3.89.91.186:8001/submit 0' \
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
        --name collator-${ACCOUNT} \
        --force-authoring \
        --ws-port ${WS_PORT} \
        --rpc-port ${RPC_PORT} \
        --telemetry-url 'ws://3.89.91.186:8001/submit 0' \
        --bootnodes /ip4/${BOOTNODE_IP}/tcp/30333/p2p/${BOOTNODE_KEY} \
        -- \
        --execution wasm \
        --chain ${RELAY_RAW} \
        --telemetry-url 'ws://3.89.91.186:8001/submit 0' \
        --name collator-side-${ACCOUNT} \
        --bootnodes /ip4/${RELAY_BOOTNODE_IP}/tcp/30333/p2p/${RELAY_BOOTNODE_KEY}
fi
