#!/bin/bash
# prerequirements for the app install
# TODO:
# if rust installed
# 		- then update?
# else
#		install rust


#Create target directory in EC2
DIR="/home/ubuntu/battlesnake"
if [ -d "$DIR" ]; then
	echo "${DIR} already exists"
else
	echo "Creating ${DIR} directory"
	mkdir ${DIR}
fi