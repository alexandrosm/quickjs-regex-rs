#!/bin/bash
export PATH="/.fly-upper-layer/.cargo/bin:$PATH"
cd /root/rebar
rebar measure -e "^quickjs/" -f "^curated/" --max-time 5s
