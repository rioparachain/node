#!/bin/sh -xe

## https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-discovery.html
#echo $AWS_INSTANCE_IPV4
#echo $AWS_INSTANCE_PORT
#echo $ECS_SERVICE_NAME
#echo $EC2_INSTANCE_ID
#echo $ECS_TASK_DEFINITION_FAMILY
#echo $ECS_TASK_SET_EXTERNAL_ID

IP_LOCAL=`ip a | sed 's,[ /], ,g' | awk '/inet 10\./{ print $2 }' | head -n 1`

curl "http://44.202.25.232:3000/relay?stage=${STAGE}" -o cur_account.txt
ACCOUNT=$(cat cur_account.txt)
BASE_PATH=/rio/keys/relay-`printf "%02d" $ACCOUNT`

# todo - change to more secure random string - get it from AWS secret store
NODE_KEY=`echo "seed ${SEED_PREFIX}//relay//$ACCOUNT" | sha256sum | sed 's,^.,0,;s, *-,,'`
ACCOUNT_PUBLIC_KEY=`echo -n ${NODE_KEY} | /rio/release/relaychain-rio key inspect-node-key --file /dev/stdin | tail -n 1`
curl "http://44.202.25.232:3000/relay/${ACCOUNT}?stage=${STAGE}" -H "content-type: application/json"  -d "{\"key\": \"${ACCOUNT_PUBLIC_KEY}\", \"ip\": \"${IP_LOCAL}\"}" -o bootnodes.json
BOOTNODE_IP=`node -e "console.log(JSON.parse(require('fs').readFileSync('bootnodes.json'))[0].ip)"`
BOOTNODE_KEY=`node -e "console.log(JSON.parse(require('fs').readFileSync('bootnodes.json'))[0].key)"`

if [ "$ACCOUNT" = "1" ]; then
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
else
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
            --rpc-port ${RPC_PORT} \
            --bootnodes /ip4/${BOOTNODE_IP}/tcp/30333/p2p/${BOOTNODE_KEY}
fi
