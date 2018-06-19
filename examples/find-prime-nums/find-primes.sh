#!/usr/bin/env bash

cd /home/ubuntu/workspace/rust-tmsn
git pull
killall rustc
killall find-prime-nums
cargo build --release
cargo run --example find-prime-nums
