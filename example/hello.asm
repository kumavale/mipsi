# Hello, World!
    .data
out_string: .asciiz "Hello, World!\n"

    .text
main:
    li $v0, 4
    la $a0, out_string
    syscall

    li $v0, 10
    syscall

