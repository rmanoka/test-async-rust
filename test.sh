#!/bin/bash

cargo build --release
for inp in 1 2 4 8 16 32 36; do
    echo "Testing with input: ${inp}"
    \time target/release/test-async-rust "$inp"
    echo
done
