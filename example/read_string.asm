
.data
    buffer: .space 32
    str1:   .asciiz "You are?\n> "
    str2:   .asciiz "Hi! "

.text
main:
    la $a0, str1    # Load and print string asking for string
    li $v0, 4
    syscall

    la $a0, buffer  # load byte space into address
    li $a1, 31      # allot the byte space for string
    li $v0, 8       # take in input

    move $t0, $a0   # save string to t0
    syscall

    la $a0, str2    # load and print "you wrote" string
    li $v0, 4
    syscall

    la $a0, buffer  # reload byte space to primary address
    move $a0, $t0   # primary address = t0 address (load pointer)
    li $v0, 4       # print string
    syscall

    li $v0, 10      # end program
    syscall

