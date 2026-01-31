#!/bin/bash
export PATH="/root/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin:$PATH"
cd /root/quickjs-regex-rs
cargo build --release 2>&1
