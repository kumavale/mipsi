
.text
main:
    prti 42        # 42
    prtn
    prth 42        # 2a
    prtn
    prtx 42        # 0x2a
    prtn
    prtc 'c'       # c
    prtn
    prts "string"  # string
    prtn

.data
char: .byte 'C'
      .align 2
str:  .asciiz "mojiretsu"

.text
	li $t0 0xff

	prti $t0          # 255
	prtn
	prth $t0          # ff
	prtn
	prtx $t0          # 0xff
	prtn
	la   $t0, str
	addi $a0, $t0, 2
    prtc $a0          # j (*(str+2))
    prtn
    prts $a0          # jiretsu
    prtn
	prtc char         # C
	prtn
    prts str          # mojiretsu
    prtn

