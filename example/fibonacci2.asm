#
# #include <stdio.h>
#
# int theArray[40];
#
# int main() {
#   int n = 2;
#   theArray[0] = 1;
#   theArray[1] = 1;
#   do {
#     theArray[n] = theArray[n-1] + theArray[n-2];
#     n++;
#   } while (n < 40);
#
#   return 0;
# }
#
#=================================================
#
# #include <stdio.h>
#
# int theArray[40];
#
# int main() {
#   int t0, t1, t2, t3, t4, t5, t6, t7;  /* Our "registers" */
#   t6 = 1;
#   t7 = 1;
#   theArray[0] = t6;  /* Storing the first two terms of the  */
#   theArray[t7] = t6; /* sequence into our array             */
#   t0 = 2;
# LLoop:
#   t3 = t0 - 2;
#   t4 = t0 - 1;
#   t1 = theArray[t3];
#   t2 = theArray[t4];
#   t5 = t1 + t2;
#   theArray[t0] = t5;
#   t0 = t0 + 1;
#   if (t0 < 40) goto LLoop;
#   return 0;
# }

    .data
theArray:
    .space 160
    .text
main:
    li    $t6, 1              # Sets t6 to 1
    li    $t7, 4              # Sets t7 to 4
    sw    $t6, theArray($0)   # Sets the first term to 1
    sw    $t6, theArray($t7)  # Sets the second term to 1
    li    $t0, 8              # Sets t0 to 8
loop:
    addi  $t3, $t0, -8
    addi  $t4, $t0, -4
    lw    $t1, theArray($t3)  # Gets the last
    lw    $t2, theArray($t4)  #   two elements
    add   $t5, $t1, $t2       # Adds them together...
    sw    $t5, theArray($t0)  # ...and stores the result
    addi  $t0, $t0, 4         # Moves to next "element" of theArray
    blt   $t0, 160, loop      # If not past the end of theArray, repeat
exit:
    li    $v0, 10
    syscall

# ----------------------------[ DATA ]----------------------------
#  0x00000000:     0x00000001  0x00000001  0x00000002  0x00000003
#  0x00000010:     0x00000005  0x00000008  0x0000000d  0x00000015
#  0x00000020:     0x00000022  0x00000037  0x00000059  0x00000090
#  0x00000030:     0x000000e9  0x00000179  0x00000262  0x000003db
#  0x00000040:     0x0000063d  0x00000a18  0x00001055  0x00001a6d
#  0x00000050:     0x00002ac2  0x0000452f  0x00006ff1  0x0000b520
#  0x00000060:     0x00012511  0x0001da31  0x0002ff42  0x0004d973
#  0x00000070:     0x0007d8b5  0x000cb228  0x00148add  0x00213d05
#  0x00000080:     0x0035c7e2  0x005704e7  0x008cccc9  0x00e3d1b0
#  0x00000090:     0x01709e79  0x02547029  0x03c50ea2  0x06197ecb
#  0x000000a0:     0x00000000  0x00000000  0x00000000  0x00000000
# ----------------------------------------------------------------

