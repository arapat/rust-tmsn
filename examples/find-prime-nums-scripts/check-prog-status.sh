#!/usr/bin/env bash

if pgrep -x "find-prime-nums" > /dev/null
then
    echo "Status: Running"
elif pgrep -x "cargo" > /dev/null
then
    echo "Status: Compiling"
else
    echo "Status: Stopped"
fi
