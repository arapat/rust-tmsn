#!/usr/bin/env bash

if pgrep -x "find-prime-nums" > /dev/null
then
    echo "Running"
elif pgrep -x "cargo" > /dev/null
then
    echo "Compiling"
else
    echo "Stopped"
fi
