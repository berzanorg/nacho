#!/bin/bash
set -e

##############################################
#               BUILD NACHO                  #
##############################################

rm -rf /nacho/bin/
mkdir -p /nacho/bin/

cargo build -p nacho-zk-rollup --release

mv ./target/release/nacho-zk-rollup /nacho/bin/


