#!/bin/bash
# Deploy to Fly.io and run rebar benchmarks.
# Usage: Run via flyctl ssh: flyctl ssh console -a rebar-bench -C "bash /root/quickjs-regex-rs/deploy.sh"
# Or source it from a remote bash session.

set -e
source /root/.cargo/env

echo "=== Pull latest code ==="
cd /root/quickjs-regex-rs
rm -f tests/date_regex.txt tests/count_bugs.rs tests/bench_date.rs tests/bench_ru.rs
git pull origin master

echo "=== Copy rebar engine ==="
cp rebar-engine/main.rs /root/rebar/engines/quickjs-regex/main.rs

echo "=== Clean build engine ==="
cd /root/rebar/engines/quickjs-regex
cargo clean
cargo build --release 2>&1 | tail -3
cp target/release/main /root/rebar/engines/quickjs/bin/main

echo "=== Run rebar ==="
cd /root/rebar
cargo run --release -- measure --verify -e quickjs/pure-rust 2>&1 | tee /root/rebar-latest.txt

echo "=== Summary ==="
echo "TIMEOUTS: $(grep -c timeout /root/rebar-latest.txt)"
echo "MISMATCHES: $(grep -c 'count mismatch' /root/rebar-latest.txt)"
grep -E "timeout|count mismatch" /root/rebar-latest.txt
