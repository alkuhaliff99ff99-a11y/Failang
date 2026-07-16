#!/data/data/com.termux/files/usr/bin/bash

set -e

echo "=== Failang v0.2 quick core preparation ==="

cargo fmt

cargo check

cargo test

cargo clippy

echo "=== Core stable ==="
