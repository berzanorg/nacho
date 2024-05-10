#!/bin/bash
set -e

##############################################
#                   SETUP                    #
##############################################

source ./scripts/environment-variables.sh

node /nacho/dependencies/node_modules/nacho-setup-script/build/index.mjs http://localhost:8080/graphql http://127.0.0.1:8181/