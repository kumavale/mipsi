
.text
main:
    li   $t0, 0x7c
    and  $a0, $t0, 0xf
    prti $a0            # 12
	prtn

    ror  $t1, $t0, 4
    and  $a0, $t1, 0xf
    prti $a0            # 7
	prtn

