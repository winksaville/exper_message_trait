#!/usr/bin/env bash

# Enable error options
set -Eeuo pipefail

# Enable debug
#set -x

cargo clean && rm -rf benches/results && cargo criterion build --target-dir benches/results && taskset -c 0 cargo criterion --target-dir benches/results