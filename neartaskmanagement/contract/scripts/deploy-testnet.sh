set -e
NETWORK=testnet
OWNER=mitsori1.$NETWORK
MASTER_ACC=mitsori1.$NETWORK
OPERATOR_ACC_SUFFIX=.meta.pool.testnet
CONTRACT_ACC=tasktracker.$MASTER_ACC
GOV_TOKEN=token.meta.$MASTER_ACC

export NEAR_ENV=$NETWORK

## delete acc
#echo "Delete $CONTRACT_ACC? are you sure? Ctrl-C to cancel"
#read input
#near delete $CONTRACT_ACC $MASTER_ACC
#near create-account $CONTRACT_ACC --masterAccount $MASTER_ACC
#
## ## redeploy code only
#near deploy $CONTRACT_ACC --wasmFile ../res/tasktracker.wasm \
# --accountId $MASTER_ACC \
# --networkId $NETWORK \
# --initFunction "new" \
#  --initArgs "{}" \
# --accountId $OWNER

#near view $CONTRACT_ACC get_next_task_id
#near view $CONTRACT_ACC increase_post_id "{}"
#near view $CONTRACT_ACC get_user_tasks '{"account_id":"mitsori1.testnet"}'

#near view $CONTRACT_ACC get_tasks '{"account_id":"bWl0c29yaTEudGVzdG5ldA=="}' --accountId $MASTER_ACC
#near view $CONTRACT_ACC generate_id "{\"account_id\":\"mitsori1.testnet\"}" --accountId $MASTER_ACC

#save this deployment  (to be able to recover state/tokens)
#set -ex
#cp ./res/tasktracker.wasm ./res/testnet/tasktracker.$CONTRACT_ACC.`date +%F.%T`.wasm
#date +%F.%T