# MIPS Interpreter
[![Actions Status](https://github.com/kumavale/mipsi/workflows/Build/badge.svg)](https://github.com/kumavale/mipsi/actions)
[![license](https://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE)  
A MIPS-32 interpreter(simulator) written in Rust.  


## How to run
If no argument, run with REPL  
```sh
cargo run --release [file...]
```

### Debug run
```sh
[ENVIRONMENT VARIABLES] cargo run --release [file...]
```  
( e.g. `TOKEN_TRACE=1 REGISTER_TRACE=1 cargo run --release file.asm` )

##### ENVIRONMENT VARIABLES
- TOKEN_TRACE  
- DATA_TRACE  
- STACK_TRACE  
- REGISTER_TRACE  
- FP_REGISTER_TRACE  

### REPL command
- exit  ... to exit
- help  ... display this message
- dispt ... display tokens
- dispd ... display data
- disps ... display stack
- dispr ... display register
- dispf ... display floating point register


## Supported
- [x] Arithmetic, Logic
    - [x] ADD[U]
    - [x] ADDI[U]
    - [x] AND
    - [x] ANDI
    - [x] CLO
    - [x] CLZ
    - [x] DIV[U]
    - [x] MULT[U]
    - [x] MUL
    - [x] MULO[U]
    - [x] MADD[U]
    - [x] MSUB[U]
    - [x] NEG[U]
    - [x] NOR
    - [x] NOT
    - [x] OR
    - [x] ORI
    - [x] REM[U]
    - [x] SLL
    - [x] SLLV
    - [x] SRA
    - [x] SRAV
    - [x] SRL
    - [x] SRLV
    - [x] ROL
    - [x] ROR
    - [x] SUB[U]
    - [x] XOR
    - [x] XORI
- [x] Constant
    - [x] LUI
    - [x] LI
- [x] Comparison
    - [x] SLT[U]
    - [x] SLTI[U]
    - [x] SEQ
    - [x] SGE[U]
    - [x] SGT[U]
    - [x] SLE[U]
    - [x] SNE
- [x] Branch
    - [x] B
    - [x] BEQ
    - [x] BGE[U]
    - [x] BGT[U]
    - [x] BLE[U]
    - [x] BLT[U]
    - [x] BNE
    - [x] BEQZ
    - [x] BGEZ
    - [x] BGTZ
    - [x] BLEZ
    - [x] BLTZ
    - [x] BNEZ
    - [x] BGEZAL
    - [x] BLTZAL
- [x] Jump
    - [x] J
    - [x] JAL
    - [x] JR
    - [x] JALR
- [ ] Load
    - [x] LA
    - [x] LB[U]
    - [x] LH[U]
    - [x] LW
- [ ] Store
    - [x] SB
    - [x] SH
    - [x] SW
- [ ] Transfer
    - [x] MOVE
    - [x] MFHI
    - [x] MFLO
    - [x] MTHI
    - [x] MTLO
    - [x] MOVN
    - [x] MOVZ
- [ ] Exception, Interrupt
    - [ ] SYSCALL
        - [x]  1: print_int
        - [x]  2: print_float
        - [ ]  3: print_double
        - [x]  4: print_string
        - [x]  5: read_int
        - [x]  6: read_float
        - [ ]  7: read_double
        - [x]  8: read_string
        - [x]  9: sbrk(allocate heap memory)
        - [x] 10: exit
        - [x] 11: print_character
        - [x] 12: read_character
        - [x] 17: exit2
        - [x] 41: random int
        - [x] 42: random int range
    - [ ] BREAK
    - [x] NOP
- [ ] Indicate
    - [x] .text
    - [x] .data
    - [x] .globl
    - [x] .align
    - [x] .word
    - [x] .half
    - [x] .byte
    - [x] .float
    - [ ] .double
    - [x] .space
    - [x] .ascii[z]
- [ ] My own
    - [x] PRTN
    - [x] PRTI
    - [x] PRTH
    - [x] PRTX
    - [x] PRTC
    - [x] PRTS
    - [x] RST
- [ ] Floating point
    - [x] LI.S
    - [x] LWC1
    - [x] SWC1
    - [x] MTC1
    - [x] ADD.S
    - [x] DIV.S
    - [x] MUL.S
    - [x] SUB.S
    - [x] CVT.S.W


## Note
1. ~~Unsigned integers are not supported, but instead behave as signed integers.~~  
2. If "Debug build", panics when overflow occurs.  

