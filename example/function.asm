
main:
	# Initialize registers
	xor		$t0,	$t0,	$t0		# $t0 = 0
	xor		$t1,	$t1,	$t1		# $t1 = 0
	addi	$t0,	$t0,	5		# $t0 = %t0 + 5
	move	$s0,	$t0				# $s0 = $t0
	move	$s1,	$t1				# $s1 = $t1

	# Call function
	move	$a0,	$s0				# Argument 1: $t0($s0)
	jal		fun						# Save current PC in $ra, and jump to fun
	move	$s1,	$v0				# Return value saved in $v0. This is y ($s1)

	# Print result ($t1)
	li		$v0,	1				# print_int syscall code = 1
	move	$a0,	$s1				# Load integer to print in $a0
	syscall							# Expect 20

	# Exit
	li		$v0,	10				# exit
	syscall

# FUNCTION: int fun(int a)
# Arguments are stored in $a0
# Return value is stored in $v0
# Return address is storedin $ra (put there by jal instruction)
fun:
	# Do the function math
	li		$s0,	3
	mul		$s1,	$s0,	$a0		# $s1 = 3 * $a0
	addi	$s1,	$s1,	5		# $s1 = $s1 + 5

	# Save the return value in $v0
	move	$v0,	$s1				# $v0 = $s1

	# Return from function
	jr		$ra						# Jump to addr stored in $ra

