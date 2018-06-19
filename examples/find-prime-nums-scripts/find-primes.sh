#!/usr/bin/env bash

cd /home/ubuntu/workspace/rust-tmsn
killall cargo
killall rustc
killall find-prime-nums
git checkout -- .
git pull
cargo build --release
RUST_LOG=debug cargo run --example find-prime-nums
