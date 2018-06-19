#!/usr/bin/env bash

cd /home/ubuntu/workspace/rust-tmsn
killall rustc
killall find-prime-nums
git checkout -- .
git pull
cargo build --release
cargo run --example find-prime-nums
