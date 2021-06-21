#!/bin/bash
## run the app

sudo apt-get install build-essential -y
curl https://sh.rustup.rs -sSf | sh -s -- -y
export PATH="$HOME/.cargo/bin:$PATH"
rustup update && cargo update

# Target directory
DIR="/home/ubuntu/battlesnake"

# switch to target
cd ${DIR}

# remove Cargo.lock just in case
rm Cargo.lock

# build the binary
echo "Building BattleSnake API"
cargo build --release
echo "Build finished"

# run on background and log into timestamped logs
N=$(date +"%Y%m%dT%H%M")
echo "Launching BattleSnake API"
cargo run --release > ${DIR}/logs/$N.out.log 2> ${DIR}/logs/$N.err.log </dev/null &
