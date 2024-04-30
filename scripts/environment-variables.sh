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
export NACHO_EVENT_FETCHER_PROCESS_SCRIPT_PATH="/nacho/dependencies/node_modules/nacho-event-fetcher-process/build/event-fetcher-process.mjs"
export NACHO_PROOF_GENERATOR_PROCESS_SCRIPT_PATH="/nacho/dependencies/node_modules/nacho-proof-generator-process/build/proof-generator-process.mjs"
export NACHO_PROOF_MERGER_PROCESS_SCRIPT_PATH="/nacho/dependencies/node_modules/nacho-proof-merger-process/build/proof-merger-process.mjs"
export NACHO_PROOF_SUBMITTER_PROCESS_SCRIPT_PATH="/nacho/dependencies/node_modules/nacho-proof-submitter-process/build/proof-submitter-process.mjs"
export NACHO_SIGNATURE_VERIFIER_PROCESS_SCRIPT_PATH="/nacho/dependencies/node_modules/nacho-signature-verifier-process/build/signature-verifier-process.mjs"
