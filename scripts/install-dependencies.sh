#!/bin/bash
set -e

##############################################
#           INSTALL DEPENDENCIES             #
##############################################

rm -rf /nacho/dependencies/
mkdir -p /nacho/dependencies/

echo '{
    "dependencies": {
        "nacho-event-fetcher-process": "0.1.0",
        "nacho-proof-generator-process": "0.1.0",
        "nacho-proof-merger-process": "0.1.0",
        "nacho-proof-submitter-process": "0.1.0",
        "nacho-signature-verifier-process": "0.1.0"
    }
}' > /nacho/dependencies/package.json

npm install --prefix /nacho/dependencies/



