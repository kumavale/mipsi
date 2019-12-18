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
* TOKEN_TRACE  
* DATA_TRACE  
* STACK_TRACE  
* REGISTER_TRACE  


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
    - [ ] LWL
    - [ ] LWR
    - [ ] LD
    - [ ] ULH[U]
    - [ ] ULW
    - [ ] LL
- [ ] Store
    - [x] SB
    - [x] SH
    - [x] SW
    - [ ] SWL
    - [ ] SWR
    - [ ] SD
    - [ ] USH
    - [ ] USW
    - [ ] SC
- [ ] Transfer
    - [x] MOVE
    - [ ] MFHI
    - [ ] MFLO
    - [ ] MTHI
    - [ ] MTLO
    - [ ] MOVN
    - [ ] MOVZ
- [ ] Exception, Interrupt
    - [ ] SYSCALL
        - [x]  1: print_int
        - [ ]  2: print_float
        - [ ]  3: print_double
        - [x]  4: print_string
        - [x]  5: read_int
        - [ ]  6: read_float
        - [ ]  7: read_double
        - [x]  8: read_string
        - [ ]  9: sbrk(allocate heap memory)
        - [x] 10: exit
        - [ ] 11: print_character
        - [ ] 12: read_character
        - [ ] 13: open file
        - [ ] 14: read from file
        - [ ] 15: write to file
        - [ ] 16: close file
        - [ ] 17: exit2
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
    - [ ] .float
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


## Note
1. ~~Unsigned integers are not supported, but instead behave as signed integers.~~  
2. If "Debug build", panics when overflow occurs.  

