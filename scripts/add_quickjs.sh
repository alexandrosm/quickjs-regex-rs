#!/bin/bash
cd /root/rebar/benchmarks/definitions/curated
for f in *.toml; do
  # Check if quickjs already in file
  if ! grep -q "quickjs" "$f"; then
    # Add quickjs engines after "engines = ["
    sed -i "s/engines = \[/engines = [\n  'quickjs\/hybrid',\n  'quickjs\/pure-rust',/" "$f"
    echo "Updated: $f"
  else
    echo "Already has quickjs: $f"
  fi
done
