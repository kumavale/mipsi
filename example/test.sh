#!/bin/bash

# Run all example
echo "cargo run ./example/arithmetic_progression.asm"
cargo run ./example/arithmetic_progression.asm || exit 1

echo "cargo run ./example/fibonacci.asm"
cargo run ./example/fibonacci.asm || exit 1

echo "cargo run ./example/fibonacci2.asm"
cargo run ./example/fibonacci2.asm || exit 1
echo "cargo run ./example/function.asm"
cargo run ./example/function.asm || exit 1

echo "cargo run ./example/hello.asm"
cargo run ./example/hello.asm || exit 1

echo "cargo run ./example/loop.asm"
cargo run ./example/loop.asm || exit 1

echo "echo '10' | cargo run ./example/non_recursive_fibonacci.asm"
echo '10' | cargo run ./example/non_recursive_fibonacci.asm || exit 1

echo "cargo run ./example/print_array_contents.asm"
cargo run ./example/print_array_contents.asm || exit 1

echo "echo 'test' | cargo run ./example/read_string.asm"
echo "test" | cargo run ./example/read_string.asm || exit 1

