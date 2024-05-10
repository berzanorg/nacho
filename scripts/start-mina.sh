#!/bin/bash
set -e


##############################################
#        STARTS MINA LAYER 1 NETWORK         #
##############################################

npm install -g zkapp-cli

zk lightnet start -p none -m single-node -b o1js-main
