#!/bin/bash
if [ $# -ne 3 ];
then
    echo "Usage $0 <PATH_TO_BATTLESNAKE> <PORT> <TIME_BUDGET>"
fi
SNAKE_DIR=$1
ARGS="-- -p $2 -t $3"

(cd ${SNAKE_DIR}; mkdir -p logs)
N=$(date +"%Y%m%dT%H%M")
echo "Launching BattleSnake API at port $2 with $3 ms time budget."
(cd ${SNAKE_DIR}; cargo run --release ${ARGS} > logs/$N.out.log 2> logs/$N.err.log </dev/null &)
sleep 1
if [ $(ps -a | grep -ic battlesnake) -ne 0 ];
then
    snakes=$(ps -a | grep -i battlesnake | awk '{ print $1 }')
    for snake in ${snakes}
    do
        address=$(netstat -tulpn | grep ${snake} | awk '{ print $4 }')
        echo "Battlesnake [${snake}] running at ${address}"
    done
else
    echo "No running Battlesnakes"
fi