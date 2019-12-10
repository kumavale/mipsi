
    .data
msg: .asciiz "Give a number: "

    .text
main:
    li $v0, 4
    la $a0, msg
    syscall

    li $v0, 5
    syscall
    add $a0, $v0, $zero

    # if ($a0 < 2) goto exit;
    blt $a0, 2, exit

    jal fib

exit:
    # Rusult
    # add $a0, $v0, $zero
    # li $v0, 1
    # syscall

    li $v0, 10
    syscall

fib:
# a0=a
# if (a==0) return 0;
# if (a==1) return 1;
#
# int x($t1), y($t2), z($t3), i($t4);
# for (x=0, y=0, z=1, i=1; i<a; ++i) {
#     x=y+z;
#     y=z;
#     z=x;
# }
#
# return(x);

    addi $t0, $zero, 1

    beqz $a0, return0
    beq $a0, $t0, return1
    jal print_output

    #arxikopiisi

    add $t1, $zero, $zero
    add $t2, $zero, $zero
    addi $t3, $zero, 1
    addi $t4, $zero, 1

    loop:
        bge $t4, $a0, exit
        add $t1, $t2, $t3
        add $t2, $zero, $t3
        add $t3, $zero, $t1
        addi $t4, $t4, 1

        jal print_output

        j loop

return0:
    add $v0, $zero, $zero
    jr $ra

return1:
    addi $v0, $zero, 1
    jr $ra

print_output:
    move $t5, $a0
    li $v0, 11
    move $a0, $t1
    syscall
    move $a0, $t5
    jr $ra

