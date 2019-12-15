#[cfg(test)]
use crate::token::*;

#[test]
#[cfg(test)]
#[allow(clippy::cognitive_complexity)]
fn test_tokenize() {
    use std::io::{BufRead, BufReader, Write};
    use super::tokenize;

    let input = "\
# This is comment.

\n
\t
\r
  
main:
    ADDI    $0,     $31,    256
    add	$t1,	$t2,	$t3
    SUB     $t4,    $t5,    $t6
    Xor     $t1,    $t1,    $t1
    LI      $v0,    1
    MOVE    $a0,    $t2
    syscall
    syscall  # Here is comment too
    BLT     $t0,$t1,label
    mul     $t4,$t5,$t6
    J       hoge
    JAL     fuga
    JR      $ra
    ($sp)   0($t0)  20($t1)
    ##### SYSCALL ##### J J J
    NOP
    ADD ADDU ADDI ADDIU SUB SUBU
    AND ANDI OR ORI XOR XORI
    B BEQ BNE
    BGE BGT BLE BLT BGEU BGTU BLEU BLTU
    BEQZ BGEZ BGTZ BLEZ BLTZ BNEZ
    SLT SLTU SLTI SLTIU SEQ SGE SGEU SGT SGTU SLE SLEU SNE
    REM REMU
    .text .data .globl main
w:  .word 42, 0, 1, 2, 3
h:  .half 3, 2, 1, 0, 42
b:  .byte 'a', 'i', 'u', 'e', 'o'
s:  .ascii \"string\"
z:  .asciiz \"stringz\"
n:  .space 256
    NOR NOT
    SLL SLLV SRA SRAV SRL SRLV
    LB LH LW
    .ascii \"string\"
    .align 2

\t\r\"literal\"\n
\"\tstr\\n\"
'C' 'h' 'a' 'r'
'\\n','\\r','\\t','\\0'
'\\'' '\\\"'

NEG NEGU SW REMU MULO MULOU
CLO CLZ ROR ROL
DIV DIVU MULT MULTU MADD MADDU MSUB MSUBU
PRTN PRTI PRTH PRTX PRTC PRTS

.word 0:0
.word 1:1
.half 2:2
.byte 4:4

.word  -2147483648, -1, 0, 1, 2147483647, 4294967295
.half  -32768, -1, 0, 1, 32767, 65535
.byte  -128, -1, -0, 0, 1, 127, 255

RST
";

    let mut tokens: Tokens = Tokens::new();
    let mut buf = String::new();
    let mut reader = BufReader::new(input.as_bytes());
    while reader.read_line(&mut buf).unwrap() > 0 {
        tokenize(0, &buf, &mut tokens);
        buf.clear();
    }

    // `cargo test -- --nocapture`
    while let Some(token) = tokens.consume() {
        print!("{:?}", token);
        std::io::stdout().flush().unwrap();
        if token.kind == TokenKind::EOL {
            println!();
        }
    }
    tokens.reset();

    assert_eq!(tokens.consume_kind(), TokenKind::LABEL("main".to_string(), 0, None));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::ADDI));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::zero,  0));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::ra,   31));
    assert_eq!(tokens.consume_kind(), TokenKind::INTEGER(256));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::ADD));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::t1,  9));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::t2, 10));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::t3, 11));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SUB));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::t4, 12));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::t5, 13));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::t6, 14));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::XOR));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::t1, 9));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::t1, 9));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::t1, 9));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::LI));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::v0, 2));
    assert_eq!(tokens.consume_kind(), TokenKind::INTEGER(1));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::MOVE));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::a0,  4));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::t2, 10));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SYSCALL));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SYSCALL));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BLT));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::t0, 8));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::t1, 9));
    assert_eq!(tokens.consume_kind(), TokenKind::ADDRESS("label".to_string()));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::MUL));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::t4, 12));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::t5, 13));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::t6, 14));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::J));
    assert_eq!(tokens.consume_kind(), TokenKind::ADDRESS("hoge".to_string()));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::JAL));
    assert_eq!(tokens.consume_kind(), TokenKind::ADDRESS("fuga".to_string()));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::JR));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::ra, 31));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::STACK(RegisterKind::sp, 29,  0));
    assert_eq!(tokens.consume_kind(), TokenKind::STACK(RegisterKind::t0,  8,  0));
    assert_eq!(tokens.consume_kind(), TokenKind::STACK(RegisterKind::t1,  9, 20));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::NOP));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::ADD));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::ADDU));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::ADDI));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::ADDIU));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SUB));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SUBU));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::AND));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::ANDI));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::OR));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::ORI));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::XOR));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::XORI));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::B));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BEQ));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BNE));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BGE));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BGT));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BLE));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BLT));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BGE));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BGT));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BLE));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BLT));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BEQZ));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BGEZ));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BGTZ));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BLEZ));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BLTZ));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BNEZ));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SLT));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SLT));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SLTI));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SLTI));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SEQ));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SGE));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SGE));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SGT));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SGT));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SLE));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SLE));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SNE));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::REM));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::REMU));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::text));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::data));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::globl("main".to_string())));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::LABEL("w".to_string(), 113, None));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::word(42)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::word(0)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::word(1)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::word(2)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::word(3)));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::LABEL("h".to_string(), 120, None));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::half(3)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::half(2)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::half(1)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::half(0)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::half(42)));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::LABEL("b".to_string(), 127, None));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(97)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(105)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(117)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(101)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(111)));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::LABEL("s".to_string(), 134, None));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::ascii("string".to_string())));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::LABEL("z".to_string(), 137, None));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::asciiz("stringz".to_string())));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::LABEL("n".to_string(), 140, None));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::space(256)));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::NOR));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::NOT));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SLL));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SLLV));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SRA));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SRAV));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SRL));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SRLV));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::LB));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::LH));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::LW));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::ascii("string".to_string())));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::align(2)));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::LITERAL("literal".to_string()));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::LITERAL("\tstr\n".to_string()));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INTEGER(67));
    assert_eq!(tokens.consume_kind(), TokenKind::INTEGER(104));
    assert_eq!(tokens.consume_kind(), TokenKind::INTEGER(97));
    assert_eq!(tokens.consume_kind(), TokenKind::INTEGER(114));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INTEGER(10));
    assert_eq!(tokens.consume_kind(), TokenKind::INTEGER(13));
    assert_eq!(tokens.consume_kind(), TokenKind::INTEGER(9));
    assert_eq!(tokens.consume_kind(), TokenKind::INTEGER(0));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INTEGER(39));
    assert_eq!(tokens.consume_kind(), TokenKind::INTEGER(34));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::NEG));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::NEGU));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SW));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::REMU));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::MULO));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::MULOU));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::CLO));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::CLZ));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::ROR));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::ROL));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::DIV));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::DIVU));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::MULT));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::MULTU));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::MADD));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::MADDU));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::MSUB));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::MSUBU));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::PRTN));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::PRTI));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::PRTH));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::PRTX));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::PRTC));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::PRTS));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::word(1)));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::half(2)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::half(2)));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(4)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(4)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(4)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(4)));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::word(2_147_483_648)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::word(4_294_967_295)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::word(0)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::word(1)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::word(2_147_483_647)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::word(4_294_967_295)));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::half(32768)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::half(65535)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::half(0)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::half(1)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::half(32767)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::half(65535)));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(128)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(255)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(0)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(0)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(1)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(127)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(255)));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::RST));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
}

#[test]
#[cfg(test)]
#[allow(clippy::cognitive_complexity)]
fn test_tokenize_hello_world() {
    use std::io::{BufRead, BufReader};
    use super::tokenize;

    let input = "\
# Hello, World!\n
.data ## Data declaration section\n
## String to be printed:\n
out_string: .asciiz \"Hello, World!\\n\"\n
.text ## Assembly language instructions go in text segment\n
main: ## Start of code section\n
    li $v0, 4           # system call code for printing string = 4\n
    la $a0, out_string  # load address of string to be printed into $a0\n
    syscall             # call operating system to perform operation\n
                        # specified in $v0\n
                        # syscall takes its arguments from $a0, $a1, ...\n
    li $v0, 10          # terminate program\n
    syscall\n
";

    let mut tokens: Tokens = Tokens::new();
    let mut buf = String::new();
    let mut reader = BufReader::new(input.as_bytes());
    while reader.read_line(&mut buf).unwrap() > 0 {
        tokenize(0, &buf, &mut tokens);
        buf.clear();
    }

    // Hello World
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::data));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::LABEL("out_string".to_string(), 2, None));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::asciiz("Hello, World!\n".to_string())));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::text));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::LABEL("main".to_string(), 7, None));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::LI));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::v0, 2));
    assert_eq!(tokens.consume_kind(), TokenKind::INTEGER(4));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::LA));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::a0, 4));
    assert_eq!(tokens.consume_kind(), TokenKind::ADDRESS("out_string".to_string()));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SYSCALL));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::LI));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::v0, 2));
    assert_eq!(tokens.consume_kind(), TokenKind::INTEGER(10));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SYSCALL));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
}

#[cfg(test)]
impl Tokens {
    pub fn consume_kind(&mut self) -> TokenKind {
        self.consume().unwrap().kind
    }
}

