#!/bin/bash

# Run all example
for file in $(ls --color=never ./example*); do
    echo
    echo ">> echo '10' | cargo run ./example/${file} || exit 1"
    echo '10' | cargo run ./example/${file} || exit 1;
done;


