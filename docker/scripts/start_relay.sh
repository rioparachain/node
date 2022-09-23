#!/bin/sh -xe

## https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-discovery.html
#echo $AWS_INSTANCE_IPV4
#echo $AWS_INSTANCE_PORT
#echo $ECS_SERVICE_NAME
#echo $EC2_INSTANCE_ID
#echo $ECS_TASK_DEFINITION_FAMILY
#echo $ECS_TASK_SET_EXTERNAL_ID

ip a
IP_LOCAL=`ip a|grep 'inet '|sed 's/^.*inet //'|sed 's/\/.*$//'|grep -E '(15|20|30)\.0\.'`
ACCOUNT=$(curl -s "${DISTRIBUTE_KEYS}/relay")
ACCOUNT_NAME=`printf "%02d" $ACCOUNT`
BASE_PATH=/rio/keys/relay-`printf "%02d" $ACCOUNT`

# todo - change to more secure random string - get it from AWS secret store
NODE_KEY=`echo "seed ${SEED_PREFIX}//relay//$ACCOUNT" | sha256sum | sed 's,^.,0,;s, *-,,'`
ACCOUNT_PUBLIC_KEY=`echo -n ${NODE_KEY} | /rio/release/relaychain-rio key inspect-node-key --file /dev/stdin | tail -n 1`

# update key-distributor
curl -X POST "${DISTRIBUTE_KEYS}/relay/${ACCOUNT}?stage=${STAGE}&ip=${IP_LOCAL}&key=${ACCOUNT_PUBLIC_KEY}"

# fetch nodes state
STATE=$(curl -X GET -s "${DISTRIBUTE_KEYS}")

RELAY_BOOTNODE_IP=`echo $STATE|jq '.[] | select(.type=="relay" and .account=="1")|.ip'|sed 's/"//g'`
RELAY_BOOTNODE_KEY=`echo $STATE|jq '.[] | select(.type=="relay" and .account=="1")|.key'|sed 's/"//g'`

if [ "$ACCOUNT" = "1" ]; then
    /rio/release/relaychain-rio \
        --base-path ${BASE_PATH} \
        --validator \
        --chain ${RELAY_RAW} \
        --node-key ${NODE_KEY} \
        --unsafe-ws-external \
        --unsafe-rpc-external \
        --no-prometheus \
        --rpc-cors all \
        --rpc-methods Unsafe \
        --name relay-${ACCOUNT_NAME} \
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
        --no-prometheus \
        --rpc-cors all \
        --rpc-methods Unsafe \
        --name relay-${ACCOUNT_NAME} \
        --ws-port ${WS_PORT} \
        --rpc-port ${RPC_PORT} \
        --bootnodes /ip4/${RELAY_BOOTNODE_IP}/tcp/30333/p2p/${RELAY_BOOTNODE_KEY}
fi
