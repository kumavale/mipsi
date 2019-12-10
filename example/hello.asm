# Hello, World!
    .data
hello: .asciiz "\nHello, "
world: .byte 'W', 'o', 'r', 'l', 'd', '!', '\n', '\0'

    .text
main:
    li $v0, 4
    la $a0, hello
    syscall
    la $a0, world
    syscall

    li $v0, 10
    syscall

