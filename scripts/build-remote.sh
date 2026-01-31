#!/bin/bash
export PATH="/root/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin:$PATH"
cd /root/quickjs-regex-rs
# Enable AVX2 for SIMD optimizations (Fly.io machines support AVX2)
RUSTFLAGS="-C target-cpu=haswell" cargo build --release 2>&1
