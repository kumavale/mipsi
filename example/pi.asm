## Calculate the approximate value of pi
## Registers used:
# t0 - holds n
# t1 - holds k
# t2 - holds exponent
# f1 - holds k as float
# f2 - holds 2k - 1 then 1/2k-1
# f3 - holds pi
.data
	inputN: .asciiz "Enter n: "
	output: .asciiz "pi = "

.text
.globl main
main:
## Ask for A
	li $t1, 1			# k = 1
	li $t2, -1			# numerator = -1
	li $f3, 0			# f3 = 0
	la $a0, inputN		# Ask for n
	li $v0, 4			# Loads print_string to $v0
	syscall

	li $v0, 5			# Read the first integer input.
	syscall

	move $t0, $v0			# t0 = n
	forloop:
		li   $t3, -1
		mul  $t2, $t3		# LO = numerator * -1
		mflo $t2    		# t2 = LO
		mtc1 $t1, $f1		# f1 holds k
		add.s $f2, $f1, $f1	# f2 = 2k
		add.s $f2, $f2, -1	# f2 = 2k - 1
		div.s $f2, $t2, $f2	# f2 = numerator/(2k-1)
		add.s $f3, $f3, $f2	# pi += f2
		addi $t1, $t1, 1	# k++
		bgt $t1, $t0, end	# if k > n, go to end
		j forloop
	end:
		mul.s $f3, $f3, 4	# pi *= 4
		la $a0, output 		# Load output string

	li $v0, 4			# Loads print_string to $v0
	syscall

	move $f12, $f3		# Load pi float
	li $v0, 2
	syscall

	li $v0, 10
	syscall
