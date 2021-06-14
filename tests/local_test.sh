#! /bin/bash

#args
if [ "$1" == "-q" ]
then
    quiet=y
else
    quiet=n
fi

s1_name="MySnakeA"
s2_name="MySnakeB"

rules_dir=/home/jiricodes/Documents/rules_battlesnake
rules_cmd="battlesnake play -W 11 -H 11 --name ${s1_name} --url http://127.0.0.1:6969 --name ${s2_name} --url http://127.0.0.1:6969 -g royale -v"

DIR=/home/jiricodes/Documents/battlesnake/
snake_bin=${DIR}Cargo.toml

num_games=5

#utils
avg_turns=0
ttl=0
a_wins=0
b_wins=0
draws=0
errors=0

#exec
pkill battlesnake
mkdir -p ${DIR}/logs
N=$(date +"%Y%m%dT%H%M")
if [ "${quiet}" != "y" ]
then
    echo "Launching BattleSnake API"
fi
cargo run --manifest-path ${snake_bin} > ${DIR}/logs/$N.out.log 2> ${DIR}/logs/$N.err.log </dev/null &
sleep 1
## run games
glog=${DIR}/logs/$N.games.log
tmplog=${DIR}/logs/tmp.log
testsum=${DIR}/logs/testsum.log
if [ "${quiet}" != "y" ]
then
    echo "Running set of ${num_games}"
fi

for ((i=1; i<=$num_games; i++))
do
    echo -ne "\nGame $i " >> $testsum
    (cd ${rules_dir}; ./${rules_cmd} 2>$tmplog)
    t=$(cat $tmplog | grep DONE | awk '{ print $7 }')
    ttl=$(( ${ttl} + $t ))
    w=$(cat $tmplog | grep DONE | awk '{ print $9 }')
    reason=""
    echo "[ $t ]" >> $testsum
    if [ "$w" == "$s1_name" ]
    then
        a_wins=$(( ${a_wins} + 1 ))
        cat $tmplog | tail -n 17 >> $glog
        reason="${s2_name}: "$(cat $tmplog | tail -n 17 | head -n 6 | grep ${s2_name} | awk '{ print $(NF-1) }')
    elif [ "$w" == "$s2_name" ]
    then
        b_wins=$(( ${b_wins} + 1 ))
        cat $tmplog | tail -n 17 >> $glog
        reason="${s1_name}: "$(cat $tmplog | tail -n 17 | head -n 6 | grep ${s1_name} | awk '{ print $(NF-1) }')
    else
        d=$(cat $tmplog | grep DONE | grep draw)
        if [ "$d" != "" ]
        then
            draws=$(( ${draws} + 1 ))
            cat $tmplog | tail -n 17 >> $glog
            reason="${s1_name}: "$(cat $tmplog | tail -n 17 | head -n 6 | grep ${s1_name} | awk '{ print $(NF-1) }')"\n""${s2_name}: "$(cat $tmplog | tail -n 17 | head -n 6 | grep ${s2_name} | awk '{ print $(NF-1) }') 
        else
            errors=$(( ${errors} + 1 ))
            cat $tmplog | tail -n 30 >> $glog
            reason=$(cat $tmplog | grep panic)
        fi
    fi
    echo -e "${reason}" >> $testsum
done

avg_turns=$(( ${ttl} / ${num_games} ))

# Final Log
echo "" >> $testsum
echo "Average Turns: ${avg_turns}" >> $testsum
echo "${s1_name}: ${a_wins}" >> $testsum
echo "${s2_name}: ${b_wins}" >> $testsum
echo "Draws: ${draws}" >> $testsum
echo "Errors: ${errors}" >> $testsum

cat $testsum >> $glog

if [ "${quiet}" != "y" ]
then
    cat $testsum
fi

# stop battlesnake api
kill %1
rm -f ${tmplog} ${testsum}