#!/bin/bash
cd /root/rebar
./target/release/rebar measure --engine quickjs --engine rust/regex --filter sherlock-en
./target/release/rebar cmp tmp/raw.csv
