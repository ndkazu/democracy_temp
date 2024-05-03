#!/usr/bin/env bash

set -e


cd $(dirname ${BASH_SOURCE[0]})/..

# Start the first process
bash -c "./target/release/node-template --dev --rpc-external "

# Wait for any process to exit
wait -n

# Exit with status of process that exited first
exit $?