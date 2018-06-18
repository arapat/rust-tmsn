#!/usr/bin/env bash

cd /home/ubuntu/workspace/rust-tmsn
git pull
cargo build --release
cargo run ---example find-prime-nums
