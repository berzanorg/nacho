#!/bin/bash
set -e

##############################################
#               START NACHO                  #
##############################################

# Don't forget to set those private environment variables before running this script.
# NACHO_RPC_SERVER_PORT
# NACHO_SUBMITTER_PRIVATE_KEY
# NACHO_MINA_NODE_URL
# NACHO_ROLLUP_CONTRACT_PUBLIC_KEY
# NACHO_BRIDGE_CONTRACT_PUBLIC_KEY

source ./scripts/environment-variables.sh

/nacho/bin/nacho-zk-rollup