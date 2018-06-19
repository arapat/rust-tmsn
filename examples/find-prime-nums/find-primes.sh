#!/usr/bin/env bash

cd /home/ubuntu/workspace/rust-tmsn
killall rustc
killall find-prime-nums
git checkout -- .
git pull
cargo build --release
RUST_LOG=info cargo run --example find-prime-nums
