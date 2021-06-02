#!/bin/bash
# Stops app in the EC2 isntance
echo "Stopping any existing battlesnake apis"
pkill battlesnake
if [ $? -eq 1 ]; then
	echo "Process not found"
fi