#!/bin/sh

BIN=$1
OUTPUT=$2
PARTIAL="par_${2}"

# this is an example of how you can perform all the tests at once
# using different binaries for each main.rs

cargo build --release --bin $BIN

for ((i=0; i<5 ; i++)); do

    echo "Iteration ${i} ${BIN}"
    { time cargo run --release --bin $BIN >/dev/null 2>&1; } 2>> $PARTIAL
done

grep "real" $PARTIAL > $OUTPUT

if [ -f $PARTIAL ]; then
    rm $PARTIAL
    echo "partial file removed"
fi