pub mod method;
pub mod register;

use super::token::register::RegisterKind;


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InstructionKind {
    /// Arithmetic, Logic
    ADD,      // Rd, Rs, Rt    | Rd = Rs + Rt
    ADDU,     // Rd, Rs, Rt    | Rd = Rs + Rt
    ADDI,     // Rt, Rs, Imm   | Rt = Rs + Imm
    ADDIU,    // Rt, Rs, Imm   | Rt = Rs + Imm
    SUB,      // Rd, Rs, Rt    | Rd = Rs - Rt
    SUBU,     // Rd, Rs, Rt    | Rd = Rs - Rt
    MUL,      // Rd, Rs, Rt    | Rd = Rs * Rt
    REM,      // Rd, Rs, Rt    | Rd = Rs % Rt
    REMU,     // Rd, Rs, Rt    | Rd = Rs % Rt

    MULO,     // Rd, Rs, Src   | Rd = Rs * Src
    MULOU,    // Rd, Rs, Src   | Rd = Rs * Src
    CLO,      // Rd, Rs        | Rd = Count leading 1 in Rs
    CLZ,      // Rd, Rs        | Rd = Count leading 0 in Rs
    ROR,      // Rd, Rs, Rt    | Rd = Shift and rotation
    ROL,      // Rd, Rs, Rt    | Rd = Shift and rotation

    DIV,      // Rd, Rs, [Rt]  | Rd = Rs / Rt  or  lo=Rd/Rs, hi=Rd%Rs
    DIVU,     // Rd, Rs, [Rt]  | Rd = Rs / Rt  or  lo=Rd/Rs, hi=Rd%Rs
    MULT,     // Rd, Rs        | lo = (Rd*Rs)[31:0], hi=(Rd*Rs)[63:32]
    MULTU,    // Rd, Rs        | lo = (Rd*Rs)[31:0], hi=(Rd*Rs)[63:32]
    MADD,     // Rd, Rs        | hi:lo += Rd * Rs
    MADDU,    // Rd, Rs        | hi:lo += Rd * Rs
    MSUB,     // Rd, Rs        | hi:lo += Rd * Rs
    MSUBU,    // Rd, Rs        | hi:lo += Rd * Rs

    NOR,      // Rd, Rs, Rt    | Rd = ~(Rs | Rt)
    NOT,      // Rd, Rs        | Rd = ~Rs
    NEG,      // Rd, Rs        | Rd = -Rs
    NEGU,     // Rd, Rs        | Rd = -Rs

    SLL,      // Rd, Rs, Shamt | Rd = Rs << Shamt
    SLLV,     // Rd, Rs, Rt    | Rd = Rs << Rt
    SRA,      // Rd, Rs, Shamt | Rd = Rs >> Shamt
    SRAV,     // Rd, Rs, Rt    | Rd = Rs >> Rt
    SRL,      // Rd, Rs, Shamt | Rd = Rs >> Shamt
    SRLV,     // Rd, Rs, Rt    | Rd = Rs >> Rt

    AND,      // Rd, Rs, Rt    | Rd = Rs & Rt
    ANDI,     // Rt, Rs, Imm   | Rt = Rs & Imm
    OR,       // Rd, Rs, Rt    | Rd = Rs | Rt
    ORI,      // Rt, Rs, Imm   | Rt = Rs | Imm
    XOR,      // Rd, Rs, Rt    | Rd = Rs ^ Rt
    XORI,     // Rt, Rs, Imm   | Rt = Rs ^ Imm

    /// Constant
    LI,       // Rd, Imm       | Rd = Imm
    LUI,      // Rt, Imm       | Rt[31:16] = Imm

    /// Comparison
    SLT,      // Rd, Rs, Rt    | Rd = if Rs < Rt  then 1 else 0
    SLTI,     // Rd, Rs, Imm   | Rd = if Rs < Imm then 1 else 0
    SEQ,      // Rd, Rs, Rt    | Rd = if Rs == Rt then 1 else 0
    SGE,      // Rd, Rs, Rt    | Rd = if Rs >= Rt then 1 else 0
    SGT,      // Rd, Rs, Rt    | Rd = if Rs = Rt  then 1 else 0
    SLE,      // Rd, Rs, Rt    | Rd = if Rs <= Rt then 1 else 0
    SNE,      // Rd, Rs, Rt    | Rd = if Rs != Rt then 1 else 0

    /// Branch
    B,        // label         | goto label
    BEQ,      // Rs, Rt, label | goto label if Rs == Rt
    BNE,      // Rs, Rt, label | goto label if Rs != Rt
    BGE,      // Rs, Rt, label | goto label if Rs >= Rt
    BGT,      // Rs, Rt, label | goto label if Rs > Rt
    BLE,      // Rs, Rt, label | goto label if Rs <= Rt
    BLT,      // Rs, Rt, label | goto label if Rs < Rt
    BEQZ,     // Rs, label     | goto label if Rs == 0
    BGEZ,     // Rs, label     | goto label if Rs >= 0
    BGTZ,     // Rs, label     | goto label if Rs > 0
    BLEZ,     // Rs, label     | goto label if Rs <= 0
    BLTZ,     // Rs, label     | goto label if Rs < 0
    BNEZ,     // Rs, label     | goto label if Rs != 0

    /// Jump
    J,        // Target        | goto Target
    JAL,      // Target        | $ra = next idx; goto Target
    JR,       // Rs, Rd        | Rd = next idx; goto Rs
    JALR,     // Rs            | goto Rs

    /// Load, Store
    LA,       // Rd, address   | Rt = idx(stack)
    LB,       // Rt, address   | Rt = stack[idx] (8bit)
    LH,       // Rt, address   | Rt = stack[idx] (16bit)
    LW,       // Rt, address   | Rt = stack[idx] (32bit)
    SW,       // Rt, address   | stack[idx] = Rt

    /// Transfer
    MOVE,     // Rd, Rs        | Rd = Rs

    /// Exception, Interrupt
    SYSCALL,  //
    NOP,      // Do nothing

    /// My own
    PRTN,     //                       | Print '\n'
    PRTI,     // Rs|literal            | Print integer
    PRTH,     // Rs|literal            | Print hex
    PRTX,     // Rs|literal            | Print hex (add 0x)
    PRTC,     // Rs|literal|label      | Print char
    PRTS,     // Rs|literal|label      | Print string

    RST,      // Reset
}

#[derive(Clone, Debug, PartialEq)]
#[allow(non_camel_case_types, dead_code)]
pub enum IndicateKind {
    text,            // Text space start
    data,            // Data space start
    globl(String),   // TODO
    word(u32),       // Number(32-bit)
    half(u16),       // (16-bit)
    byte(u8),        // 1 char(8-bit)
    space(u32),      // n byte
    ascii(String),   // String
    asciiz(String),  // String
    align(u8),       // Align
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    INSTRUCTION(InstructionKind),         // Instruction
    INDICATE(IndicateKind),               // Indication
    INTEGER(i32),                         // Immediate
    REGISTER(RegisterKind, usize),        // (_, Index)
    STACK(RegisterKind, usize, i32),      // (_, Append index)
    DATA(RegisterKind, usize, String),    // (_, Label name)
    LABEL(String, usize, Option<usize>),  // (Literal, Token index, Data index)
    ADDRESS(String),                      // Label
    LITERAL(String),                      // Literal
    INVALID(String),                      // Invalid string
    EOL,                                  // End of Line
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,  // Token kind
    pub line: u32,        // Number of lines
}

#[derive(Debug)]
pub struct Tokens {
    pub token: Vec<Token>,          // Token's vector
    pub data_area_now: bool,        // for data_analysis() in REPL

    idx: usize,                     // Current index
    foremost: bool,                 // Foremost
    length: usize,                  // Token length
    addresses: Vec<(String, usize)> // (label name, token index)
}

