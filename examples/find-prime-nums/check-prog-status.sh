#!/usr/bin/env bash

if pgrep -x "find-prime-nums" > /dev/null
then
    echo "Running"
else
    echo "Stopped"
fi