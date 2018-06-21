#!/usr/bin/env bash

cd /home/ubuntu/workspace/rust-tmsn
killall cargo 2> /dev/null
killall rustc 2> /dev/null
killall find-prime-nums 2> /dev/null
git checkout -- .
git pull
cargo build --release 2> /dev/null
RUST_LOG=debug,cargo=error cargo run --example find-prime-nums
