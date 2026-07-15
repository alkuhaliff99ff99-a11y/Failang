#!/bin/bash

set -e

echo "=== Failang Language Tests ==="

for f in tests/language/*.fsl; do
    echo "--- $f ---"
    cargo run "$f"
done

echo "=== All language tests completed ==="
