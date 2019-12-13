
.text
main:
    li $t0, 0x7c
    and $a0, $t0, 0xf
    li $v0, 128
    syscall            # 12

    ror $t1, $t0, 4
    and $a0, $t1, 0xf
    li $v0, 128
    syscall            # 7

