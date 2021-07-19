#! /bin/bash

num_games=20

#args
if [ "$1" == "-q" ]
then
    quiet=y
else
    quiet=n
fi

# Adversary number 1
s1_name="V0.1.1"
s1_proc="minimax-v0.1.1"
s1_bin="./${s1_proc} -t 400 -p 6970"
s1_add="http://127.0.0.1:6970"
s1_log="logs-${s1_name}"

# My current snake
my_name="CurrentWork"
my_add="http://127.0.0.1:6969"
my_dir=/home/jiricodes/Documents/battlesnake/
my_bin=${my_dir}Cargo.toml
my_args="-p 6969 -t 400"


rules_dir=/home/jiricodes/Documents/rules_battlesnake_myfork
rules_cmd="battlesnake play -W 11 -H 11 --name ${s1_name} --url ${s1_add} --name ${my_name} --url ${my_add} -g royale -v"

#utils
avg_turns=0
ttl=0
a_wins=0
b_wins=0
draws=0
errors=0

#Stop running instances
pkill battlesnake
pkill ${s1_proc}

# Created logging directory if not existent
mkdir -p ${my_dir}/logs

# Save start time
N=$(date +"%Y%m%dT%H%M")

# Start adversaries
if [ "${quiet}" != "y" ]
then
    echo "Launching Adversaries"
fi

# Create adversary 1 logging function
mkdir -p ${s1_log}
# Start adversary 1
${s1_bin} > ${s1_log}/$N.out.log 2> ${s1_log}/$N.err.log </dev/null &
sleep 1

# Start current work
if [ "${quiet}" != "y" ]
then
    echo "Launching Current BattleSnake API"
fi

(cd ${my_dir}; cargo build --release);
sleep 1
cargo run --release --manifest-path ${my_bin} -- ${my_args} > ${my_dir}/logs/$N.out.log 2> ${my_dir}/logs/$N.err.log </dev/null &
sleep 1
## run games
glog=${my_dir}/logs/$N.games.log
tmplog=${my_dir}/logs/tmp.log
testsum=${my_dir}/logs/testsum.log
if [ "${quiet}" != "y" ]
then
    echo "Running set of ${num_games}"
fi

for ((i=1; i<=$num_games; i++))
do
    echo -ne "\n ----------------- Game $i --------------" >> ${my_dir}/logs/$N.out.log
    echo -ne "\nGame $i " >> ${testsum}
    echo -ne "Game $i "
    (cd ${rules_dir}; ./${rules_cmd} 2>${tmplog})
    t=$(cat ${tmplog} | grep DONE | awk '{ print $7 }')
    w=$(cat ${tmplog} | grep DONE | awk '{ print $9 }')
    reason=""
    
    if [ "$w" == "$s1_name" ]
    then
        a_wins=$(( ${a_wins} + 1 ))
        cat ${tmplog} | tail -n 17 >> ${glog}
        reason="${my_name}: "$(cat ${tmplog} | tail -n 17 | head -n 6 | grep ${my_name} | awk '{ print $(NF-1) }')
    elif [ "$w" == "$my_name" ]
    then
        b_wins=$(( ${b_wins} + 1 ))
        cat ${tmplog} | tail -n 17 >> ${glog}
        reason="${s1_name}: "$(cat ${tmplog} | tail -n 17 | head -n 6 | grep ${s1_name} | awk '{ print $(NF-1) }')
    else
        d=$(cat ${tmplog} | grep DONE | grep draw)
        if [ "$d" != "" ]
        then
            draws=$(( ${draws} + 1 ))
            cat ${tmplog} | tail -n 17 >> ${glog}
            reason="${s1_name}: "$(cat ${tmplog} | tail -n 17 | head -n 6 | grep ${s1_name} | awk '{ print $(NF-1) }')"\n""${my_name}: "$(cat ${tmplog} | tail -n 17 | head -n 6 | grep ${my_name} | awk '{ print $(NF-1) }') 
        else
            errors=$(( ${errors} + 1 ))
            cat ${tmplog} | tail -n 33 >> ${glog}
            reason=$(cat ${tmplog} | grep panic)
            t=$(cat ${tmplog} | tail -n 33 | head -n 1 | awk '{ print $(NF) }')
        fi
    fi
    echo "[ $t ]" >> ${testsum}
    echo -n "[ $t ] Death: "
    printf -v ti '%d\n' $t 2>/dev/null
    ttl=$(( ${ttl} + ${ti} ))
    echo -e "${reason}" >> ${testsum}
    echo -e "${reason}"
    echo "" >> ${glog}
done

avg_turns=$(( ${ttl} / ${num_games} ))

# Final Log
echo "" >> ${testsum}
echo "Average Turns: ${avg_turns}" >> ${testsum}
echo "${s1_name}: ${a_wins}" >> ${testsum}
echo "${my_name}: ${b_wins}" >> ${testsum}
echo "Draws: ${draws}" >> ${testsum}
echo "Errors: ${errors}" >> ${testsum}

cat ${testsum} >> ${glog}

if [ "${quiet}" != "y" ]
then
    cat ${testsum}
fi

# stop battlesnake api
pkill battlesnake
pkill ${s1_proc}
rm -f ${tmplog} ${testsum}