#!/bin/bash

# Run all example
cargo run ./example/arithmetic_progression.asm
cargo run ./example/fibonacci.asm
cargo run ./example/fibonacci2.asm
cargo run ./example/function.asm
cargo run ./example/hello.asm
cargo run ./example/loop.asm
echo "10" | cargo run ./example/non_recursive_fibonacci.asm
cargo run ./example/print_array_contents.asm
echo "test" | cargo run ./example/read_string.asm

