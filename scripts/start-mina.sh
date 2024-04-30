#!/bin/bash
set -e


##############################################
#                START MINA                  #
##############################################

npm install -g zkapp-cli

zk lightnet start
