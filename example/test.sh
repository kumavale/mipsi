#!/bin/bash

# Run all example
cargo run ./arithmetic_progression.asm
cargo run ./fibonacci.asm
cargo run ./fibonacci2.asm
cargo run ./function.asm
cargo run ./hello.asm
cargo run ./loop.asm
echo "10" | cargo run ./non_recursive_fibonacci.asm
cargo run ./print_array_contents.asm
echo "test" | cargo run ./read_string.asm
