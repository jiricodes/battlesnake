#!/bin/bash
## run the app

# Target directory
DIR="/home/ubuntu/battlesnake"

# switch to target
cd ${DIR}

# build the binary
echo "Building BattleSnake API"
cargo build
echo "Build finished"

# run on background and log into timestamped logs
N=$(date +"%Y%m%dT%H%M")
echo "Launching BattleSnake API"
cargo run > ${DIR}/logs/$N.out.log 2> ${DIR}/logs/$N.err.log </dev/null &
