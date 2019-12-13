
.text
main:
    li $t0, 124
    and $a0, $t0, 15
    li $v0, 128
    syscall            # 12

    ror $t1, $t0, 4
    and $a0, $t1, 15
    li $v0, 128
    syscall            # 7

