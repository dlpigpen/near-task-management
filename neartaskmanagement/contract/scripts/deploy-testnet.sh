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
#meta deploy ./res/metapool.wasm
#meta new { owner_account_id:$OWNER, treasury_account_id:treasury$OPERATOR_ACC_SUFFIX, operator_account_id:operator$OPERATOR_ACC_SUFFIX, meta_token_account_id:$GOV_TOKEN } --accountId $MASTER_ACC
## set params@meta set_params
#meta set_params
## deafult 4 pools
##meta default_pools_testnet

## test
#near call $CONTRACT_ACC set_busy "{\"value\":false}" --accountId $CONTRACT_ACC --depositYocto 1

# ## redeploy code only
#near deploy $CONTRACT_ACC --wasmFile ../res/tasktracker.wasm  --accountId $MASTER_ACC --networkId $NETWORK
# ## MIGRATE
#near call $CONTRACT_ACC migrate "{}" --accountId $CONTRACT_ACC

#near deploy contract4.preprod-pool.testnet ./res/metapool.wasm  --accountId preprod-pool.testnet
#near call contract4.preprod-pool.testnet set_busy "{\"value\":false}" --accountId preprod-pool.testnet --depositYocto 1
near view $CONTRACT_ACC get_next_task_id "{}"
near view $CONTRACT_ACC increase_post_id "{}"

#near view $CONTRACT_ACC get_tasks '{"account_id":"bWl0c29yaTEudGVzdG5ldA=="}' --accountId $MASTER_ACC
#near view $CONTRACT_ACC generate_id "{\"account_id\":\"mitsori1.testnet\"}" --accountId $MASTER_ACC

#save this deployment  (to be able to recover state/tokens)
#set -ex
#cp ./res/metapool.wasm ./res/testnet/tasktracker.$CONTRACT_ACC.`date +%F.%T`.wasm
#date +%F.%T