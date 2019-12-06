# 1+2+3+4+5+6+7+8+9+10
# expect: r9 = 55

# 0 initialization
XOR		r0,		r0,		r0
# calc 1 item
ADDI	r8,		r0,		1
ADD		r9,		r0,		r8
# calc 2 items (loop)
ADDI	r8,		r8,		1
ADD		r9,		r8,		r9
ADDI	r8,		r8,		1
ADD		r9,		r8,		r9
ADDI	r8,		r8,		1
ADD		r9,		r8,		r9
ADDI	r8,		r8,		1
ADD		r9,		r8,		r9
ADDI	r8,		r8,		1
ADD		r9,		r8,		r9
ADDI	r8,		r8,		1
ADD		r9,		r8,		r9
ADDI	r8,		r8,		1
ADD		r9,		r8,		r9
ADDI	r8,		r8,		1
ADD		r9,		r8,		r9
# calc 10 item
ADDI	r8,		r8,		1
ADD		r9,		r8,		r9

