pub mod method;
pub mod register;
pub mod memory;

use super::token::register::RegisterKind;


#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
pub enum InstructionKind {
    /// # CPU Instructions
    /// Arithmetic, Logic
    ADD,      // Rd, Rs, Rt    | Rd = Rs + Rt
    ADDU,     // Rd, Rs, Rt    | Rd = Rs + Rt (without overflow)
    ADDI,     // Rt, Rs, Imm   | Rt = Rs + Imm
    ADDIU,    // Rt, Rs, Imm   | Rt = Rs + Imm (without overflow)
    SUB,      // Rd, Rs, Rt    | Rd = Rs - Rt
    SUBU,     // Rd, Rs, Rt    | Rd = Rs - Rt (without overflow)
    MUL,      // Rd, Rs, Rt    | Rd = Rs * Rt (without overflow)
    REM,      // Rd, Rs, Rt    | Rd = Rs % Rt
    REMU,     // Rd, Rs, Rt    | Rd = Rs % Rt (without overflow)

    MULO,     // Rd, Rs, Src   | Rd = Rs * Src (with overflow)
    MULOU,    // Rd, Rs, Src   | Rd = Rs * Src (unsigned with overflow)
    CLO,      // Rd, Rs        | Rd = Count leading 1 in Rs
    CLZ,      // Rd, Rs        | Rd = Count leading 0 in Rs
    ROR,      // Rd, Rs, Rt    | Rd = Shift and rotation
    ROL,      // Rd, Rs, Rt    | Rd = Shift and rotation

    DIV,      // Rd, Rs, [Rt]  | Rd = Rs / Rt  or  lo=Rd/Rs, hi=Rd%Rs
    DIVU,     // Rd, Rs, [Rt]  | Rd = Rs / Rt  or  lo=Rd/Rs, hi=Rd%Rs (unsigned)
    MULT,     // Rd, Rs        | lo = (Rd*Rs)[31:0], hi=(Rd*Rs)[63:32]
    MULTU,    // Rd, Rs        | lo = (Rd*Rs)[31:0], hi=(Rd*Rs)[63:32]
    MADD,     // Rd, Rs        | hi:lo += Rd * Rs
    MADDU,    // Rd, Rs        | hi:lo += Rd * Rs
    MSUB,     // Rd, Rs        | hi:lo += Rd * Rs
    MSUBU,    // Rd, Rs        | hi:lo += Rd * Rs

    NOR,      // Rd, Rs, Rt    | Rd = ~(Rs | Rt)
    NOT,      // Rd, Rs        | Rd = ~Rs
    NEG,      // Rd, Rs        | Rd = -Rs (with overflow)
    NEGU,     // Rd, Rs        | Rd = -Rs (without overflow)

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
    BGEZAL,   // Rs, label     | $ra = next idx; goto label if Rs >= 0
    BLTZAL,   // Rs, label     | $ra = next idx; goto label if Rs < 0

    /// Jump
    J,        // Target        | goto Target
    JAL,      // Target        | $ra = next idx; goto Target
    JR,       // Rs, Rd        | Rd = next idx; goto Rs
    JALR,     // Rs            | goto Rs

    /// Load
    LA,       // Rd, address   | Rt = idx(stack)
    LB,       // Rt, address   | Rt = stack[idx] (8bit)
    LBU,      // Rt, address   | Rt = stack[idx] (8bit)
    LH,       // Rt, address   | Rt = stack[idx] (16bit)
    LHU,      // Rt, address   | Rt = stack[idx] (16bit)
    LW,       // Rt, address   | Rt = stack[idx] (32bit)

    /// Store
    SB,       // Rt, address   | stack[idx] = Rt (8bit)
    SH,       // Rt, address   | stack[idx] = Rt (16bit)
    SW,       // Rt, address   | stack[idx] = Rt (32bit)

    /// Transfer
    MOVE,     // Rd, Rs        | Rd = Rs
    MFHI,     // Rd            | Rd = hi
    MFLO,     // Rd            | Rd = lo
    MTHI,     // Rs            | hi = Rs
    MTLO,     // Rs            | lo = Rs
    MOVN,     // Rd, Rs, Rt    | Rd = Rs if Rt != 0
    MOVZ,     // Rd, Rs, Rt    | Rd = Rs if Rt == 0

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

    /// # FPU Instructions
    /// Loads and Stores Using Register+Offset Address Mode
    //LDC1,
    //LWC1,
    //SDC1,
    //SWC1,

    /// Move To and From Instructions
    //CFC1,
    //CTC1,
    //MFC1,
    MTC1,

    /// Arithmetic Instructions
    ABS_S,
    ADD_S,
    DIV_S,
    //MADD_S,
    //MSUB_S,
    MUL_S,
    NEG_S,
    //NMADD_S,
    //NMSUB_S,
    //RECIP_S,
    //RSQRT_S,
    //SQRT,
    SUB_S,

    /// Branch Instructions
    BC1F,
    BC1T,

    /// Compare Instructions
    C_EQ_S,
    C_LE_S,
    C_LT_S,

    /// Convert Instructions
    CVT_S_W,
    CVT_W_S,
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
    float(f32),      // single floating point (32-bit)
    space(u32),      // n byte
    ascii(String),   // String
    asciiz(String),  // String
    align(u8),       // Align
}

#[derive(Clone, Debug, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub enum TokenKind {
    INSTRUCTION(InstructionKind),         // Instruction
    INDICATE(IndicateKind),               // Indication
    INTEGER(i32),                         // integer immediate
    FLOATING(f32),                        // floating point immediate
    REGISTER(RegisterKind, usize),        // (_, Register index)
    MEMORY(RegisterKind, usize, u32),     // (_, Register index, Append index) for data,stack
    DATA(RegisterKind, usize, String),    // (_, Label name)
    LABEL(String, usize, Option<usize>),  // (Literal, Token index, Data index)
    ADDRESS(String),                      // Label
    LITERAL(String),                      // Literal
    EOL,                                  // End of Line
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,  // Token kind
    pub line: u32,        // Number of lines
    filename_idx: usize,  // File name index
}

#[derive(Debug)]
pub struct Tokens {
    pub token: Vec<Token>,            // Token's vector
    pub data_area_now: bool,          // for data_analysis() in REPL

    idx: usize,                       // Current index
    foremost: bool,                   // Foremost
    length: usize,                    // Token length
    addresses: Vec<(String, usize)>,  // (label name, token index)
    filenames: Vec<String>,           // filenames

    token_trace: bool,                // Environment variable 'TOKEN_TRACE'
    data_trace: bool,                 // Environment variable 'DATA_TRACE'
    stack_trace: bool,                // Environment variable 'STACK_TRACE'
    register_trace: bool,             // Environment variable 'REGISTER_TRACE'
    fp_register_trace: bool,          // Environment variable 'FP_REGISTER_TRACE'
}

pub static CONSUME_ERR: &str = "token.consume(): none";

