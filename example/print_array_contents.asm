
        .data
list:   .word   1, 2, 3, 4, 6, 8, 11, 13, 16, 18, 26  # Ulam number
size:   .word   11
NL:     .byte   '\n'

        .text
main:
        lw      $t3, size
		la      $t1, list        # get array address
		li      $t2, 0           # set loop counter

prnlp:
		beq     $t2, $t3, exit   # print list element

		lw      $a0, ($t1)
		li      $v0, 1
		syscall

		la      $a0, NL          # print a newline
		li      $v0, 4
		syscall

		addi    $t2, $t2, 1      # advance loop counter
		addi    $t1, $t1, 4      # advance array pointer
		b       prnlp            # repeat the loop

exit:
		la      $a0, NL          # print a newline
		li      $v0, 4
		syscall

