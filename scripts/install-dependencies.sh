#!/bin/bash
set -e

##############################################
#           INSTALL DEPENDENCIES             #
##############################################

rm -rf /nacho/dependencies/
mkdir -p /nacho/dependencies/

echo '{
    "dependencies": {
        "nacho-event-fetcher-process": "1.0.0",
        "nacho-proof-generator-process": "1.0.0",
        "nacho-proof-merger-process": "1.0.0",
        "nacho-proof-submitter-process": "1.0.0",
        "nacho-setup-script": "1.0.3",
        "nacho-signature-verifier-process": "1.0.0"
    }
}' > /nacho/dependencies/package.json

npm install --prefix /nacho/dependencies/



