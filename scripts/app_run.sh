#!/bin/bash
## run the app

# Target directory
DIR="/home/ubuntu/battlesnake"

# switch to target
cd ${DIR}

# build the binary
cargo build

# run on background and log into timestamped logs
N=$(date +"%Y%m%dT%H%M")
cargo run > logs/$N.out.log 2> logs/$N.err.log < /dev/null &