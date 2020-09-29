#!/bin/bash

# User port X0000
# 1xxx for each type (relay vs parachain)
# 1xx for each node
# 42 for p2p
# 43 for http
# 44 for ws
#
# Ex: USER_PORT=20000 scripts/run-parachain.sh
# Will open port 21042, 21043, 21044

# Loading binary/specs variables
source scripts/_init_var.sh

# We retrieve the list of relay node for 
RELAY_PORT=$((USER_PORT + 42))
RELAY_INDEX=0
RELAY_BOOTNODES_ARGS=""

while nc -z -v -w5 ${RELAY_IP} ${RELAY_PORT} 2> /dev/null
do 
    echo "Found existing relay on ${RELAY_PORT}."
    RELAY_BOOTNODES_ARGS="$RELAY_BOOTNODES_ARGS --bootnodes /ip4/$RELAY_IP/tcp/${RELAY_PORT}/p2p/${RELAY_LOCAL_IDS[$RELAY_INDEX]}"
    RELAY_INDEX=$((RELAY_INDEX + 1))
    RELAY_PORT=$((RELAY_PORT + 100))
    
    if [ $RELAY_PORT -ge $((USER_PORT + 1000)) ]
    then
        break
    fi
done


PARACHAIN_PORT=$((USER_PORT + 1000 + 42))
PARACHAIN_INDEX=0
PARACHAIN_BOOTNODES_ARGS=""
while nc -z -v -w5 ${PARACHAIN_IP} ${PARACHAIN_PORT} 2> /dev/null
do 
    echo "Found existing parachain on ${PARACHAIN_PORT}."
    PARACHAIN_BOOTNODES_ARGS="$PARACHAIN_BOOTNODES_ARGS --bootnodes /ip4/$PARACHAIN_IP/tcp/${PARACHAIN_PORT}/p2p/${PARACHAIN_LOCAL_IDS[$PARACHAIN_INDEX]}"
    PARACHAIN_INDEX=$((PARACHAIN_INDEX + 1))
    PARACHAIN_PORT=$((PARACHAIN_PORT + 100))
    
    if [ $PARACHAIN_PORT -ge $((USER_PORT + 2000)) ]
    then
        echo "No more parachain port available! (limited to 9 parachains)"
        exit 1
    fi
done


echo "parachain $PARACHAIN_INDEX ($PARACHAIN_ID) - p2p-port: $PARACHAIN_PORT, http-port: $((PARACHAIN_PORT + 1)) , ws-port: $((PARACHAIN_PORT + 2))"

$PARACHAIN_BINARY \
    --node-key ${PARACHAIN_KEYS[$PARACHAIN_INDEX]} \
    --port $((PARACHAIN_PORT + 10)) \
    --rpc-port $((PARACHAIN_PORT + 10 + 1)) \
    --ws-port $((PARACHAIN_PORT + 10 + 2)) \
    --validator \
    --tmp \
    '-linfo,evm=trace,ethereum=trace,rpc=trace' \
    --chain $PARACHAIN_SPEC_PLAIN  \
    $PARACHAIN_BOOTNODES_ARGS \
    -- \
      --node-key ${PARACHAIN_KEYS[$PARACHAIN_INDEX]} \
      --tmp \
      --port $((PARACHAIN_PORT)) \
      --rpc-port $((PARACHAIN_PORT + 1)) \
      --ws-port $((PARACHAIN_PORT + 2)) \
      $RELAY_BOOTNODES_ARGS \
      --chain $POLKADOT_SPEC_RAW;
