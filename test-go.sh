#!/bin/bash

go build fib.go
for inp in 1 2 4 8 16 32 36; do
    echo "Testing with input: ${inp}"
    \time ./fib "$inp"
    echo
done
