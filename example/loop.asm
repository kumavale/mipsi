# i = 1;
# while i < 5 {
#     i = i + 1;
# }
# printf("%d", i);  // 5

main:
	xor		$t0		$t0		$t0		# $t0 = 0
	xor		$t1		$t1		$t1		# $t1 = 0
	addi	$t1		$t1		5		# $t1 = %t1 + 5
loop:
	addi	$t0		$t0		1       # $t0 = $t0 + 1
	blt		$t0		$t1		loop    # goto loop if $t0 < $t1

# print result
	li		$v0,	1
	move	$a0,	$t0
	syscall
