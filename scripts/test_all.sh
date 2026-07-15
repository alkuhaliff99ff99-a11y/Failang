#!/bin/bash

set -e

echo "=== Failang Full Test Suite ==="

for dir in tests/language tests/runtime; do
    if [ -d "$dir" ]; then
        for f in "$dir"/*.fsl; do
            [ -e "$f" ] || continue
            echo "--- $f ---"
            cargo run "$f"
        done
    fi
done

echo "=== All tests passed ==="
