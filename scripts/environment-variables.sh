#!/bin/bash
set -e

##############################################
#        ENVIRONMENT VARIABLES               #
##############################################

export NACHO_PROOFS_PATH="/nacho/state/proofs/"
export NACHO_BALANCES_DB_PATH="/nacho/state/balances-db/"
export NACHO_LIQUIDITIES_DB_PATH="/nacho/state/liquidities-db/"
export NACHO_POOLS_DB_PATH="/nacho/state/pools-db/"
export NACHO_BURNS_DB_PATH="/nacho/state/burns-db/"
export NACHO_WITHRAWALS_DB_PATH="/nacho/state/withdrawals-db/"
export NACHO_TRANSACTIONS_DB_PATH="/nacho/state/transactions-db/"
export NACHO_EVENTS_DB_PATH="/nacho/state/events-db/"
export NACHO_MEMPOOL_PATH="/nacho/state/mempool/"
export NACHO_PROOFPOOL_PATH="/nacho/state/proofpool/"
export NACHO_EVENT_FETCHER_PROCESS_SCRIPT_PATH="/nacho/dependencies/node_modules/nacho-event-fetcher-process/build/index.mjs"
export NACHO_PROOF_GENERATOR_PROCESS_SCRIPT_PATH="/nacho/dependencies/node_modules/nacho-proof-generator-process/build/index.mjs"
export NACHO_PROOF_MERGER_PROCESS_SCRIPT_PATH="/nacho/dependencies/node_modules/nacho-proof-merger-process/build/index.mjs"
export NACHO_PROOF_SUBMITTER_PROCESS_SCRIPT_PATH="/nacho/dependencies/node_modules/nacho-proof-submitter-process/build/index.mjs"
export NACHO_SIGNATURE_VERIFIER_PROCESS_SCRIPT_PATH="/nacho/dependencies/node_modules/nacho-signature-verifier-process/build/index.mjs"

# Don't forget to set those private environment variables before running this script.
# export NACHO_RPC_SERVER_PORT=""
# export NACHO_SUBMITTER_PRIVATE_KEY=""
# export NACHO_MINA_GRAPHQL_URL=""
# export NACHO_MINA_ARCHIVE_URL=""
# export NACHO_ROLLUP_CONTRACT_PUBLIC_KEY=""
# export NACHO_BRIDGE_CONTRACT_PUBLIC_KEY=""