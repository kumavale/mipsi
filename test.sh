#!/bin/bash

# Run all examples
echo ">> cargo build --release"
cargo build --release

for file in $(ls --color=never ./example*); do
    echo
    echo ">> echo '10' | cargo run --release ./example/${file} || exit 1"
    echo '10' | cargo run --release ./example/${file} || exit 1;
done;

