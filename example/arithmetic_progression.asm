# 1+2+3+4+5+6+7+8+9+10
# expect: $t2 = 55

# 0 initialization
XOR		$t0,	$t0,	$t0
# calc 1 item
ADDI	$t1,	$t0,	1
ADD		$t2,	$t0,	$t1
# calc 2 items (loop)
ADDI	$t1,	$t1,	1
ADD		$t2,	$t1,	$t2
ADDI	$t1,	$t1,	1
ADD		$t2,	$t1,	$t2
ADDI	$t1,	$t1,	1
ADD		$t2,	$t1,	$t2
ADDI	$t1,	$t1,	1
ADD		$t2,	$t1,	$t2
ADDI	$t1,	$t1,	1
ADD		$t2,	$t1,	$t2
ADDI	$t1,	$t1,	1
ADD		$t2,	$t1,	$t2
ADDI	$t1,	$t1,	1
ADD		$t2,	$t1,	$t2
ADDI	$t1,	$t1,	1
ADD		$t2,	$t1,	$t2
# calc 10 item
ADDI	$t1,	$t1,	1
ADD		$t2,	$t1,	$t2

