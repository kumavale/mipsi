
# ```
# #include <stdio.h>
# int fibonacci(int i);
#
# int main(int argc, char **argv) {
# 	for (int i=0; i<16; ++i) {
# 	    int fib = fibonacci(i);
# 		printf("%d\n", fib);
# 	}
#
# 	return 0;
# }
#
# int fibonacci(int n) {
#     if (n <= 1) return n;
# 	return fibonacci(n-1) + fibonacci(n-2);
# }
# ```

main:
    # loop from $t0 = 0 until 16
    move $t0, $0
    loop: beq $t0, 16, exit

    jal fibonacci
    jal print_output

    addi $t0, $t0, 1
    j loop

exit:
	li $v0, 10
	syscall

print_output:
    li $v0, 11
    move $a0, $t1
    syscall
    jr $ra

fibonacci:
    move $t1, $0
    move $t2, $sp
    li $t3, 1
    addi $sp, $sp, -4               # push initial $t0 on stack
    sw $t0, 0($sp)

    recursive_call:
        beq $sp, $t2, fib_exit      # if stack is empty, exit
        lw $t4, 0($sp)              # pop next $t4 off stack
        addi $sp, $sp, 4
        ble $t4, $t3, early_return

        sub $t4, $t4, 1             # push $t4 - 1 on stack
        addi $sp, $sp, -4
        sw $t4, 0($sp)

        sub $t4, $t4, 1             # push $t4 - 2 on stack
        addi $sp, $sp, -4
        sw $t4, 0($sp)

        j recursive_call

    early_return:

        add $t1, $t1, $t4
        j recursive_call

    fib_exit:
        jr $ra

