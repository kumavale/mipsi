#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InstructionKind {
    /// Arithmetic, Logic
    ADD,      // Rd, Rs, Rt    | Rd = Rs + Rt
    ADDI,     // Rt, Rs, Imm   | Rt = Rs + Imm
    SUB,      // Rd, Rs, Rt    | Rd = Rs - Rt
    MUL,      // Rd, Rs, Rt    | Rd = Rs * Rt
    DIV,      // Rd, Rs, Rt    | Rd = Rs / Rt

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
    LW,       // Rt, address   | Rt = stack[idx]
    SW,       // Rt, address   | stack[idx] = Rt

    /// Transfer
    MOVE,     // Rd, Rs        | Rd = Rs

    /// Exception, Interrupt
    SYSCALL,  //
    NOP,      // Do nothing
}

#[derive(Clone, Debug, PartialEq)]
#[allow(non_camel_case_types, dead_code)]
pub enum IndicateKind {
    text,            // Text space start
    data,            // Data space start
    globl,           // Ignore
    word(i32),       // Number(32-bit)
    byte(char),      // 1 char(8-bit)
    space,           // n byte
    asciiz(String),  // Strings
}

#[derive(Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum RegisterKind {
    zero,                            //     0: Hard-wired to 0
    at,                              //     1: Reserved for pseudo-instructions
    v0, v1,                          //   2-3: Return values from functions
    a0, a1, a2, a3,                  //   4-7: Arguments to functions - not preserved by subprograms
    t0, t1, t2, t3, t4, t5, t6, t7,  //  8-15: Temporary data, not preserved by subprograms
    s0, s1, s2, s3, s4, s5, s6, s7,  // 16-23: Saved registers, preserved by subprograms
    t8, t9,                          // 24-25: More temporary registers, not preserved by subprograms
    k0, k1,                          // 26-27: Reserved for kernel. Do not use.
    gp,                              //    28: Global Area Pointer (base of global data segment)
    sp,                              //    29: Stack Pointer
    fp,                              //    30: Frame Pointer
    ra,                              //    31: Return Address
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    INSTRUCTION(InstructionKind),
    //INDICATE(IndicateKind),           // Pseudo instruction
    INTEGER(i32),                     // Immediate
    REGISTER(RegisterKind, usize),    // (_, Index)
    STACK(RegisterKind, usize, i32),  // (_, Append index)
    LABEL(String, usize),             // (Literal, Index)
    ADDRESS(String),                  // Literal
    EOL,                              // End of Line
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,  // Token kind
    pub line: u32,        // Number of lines
}

#[derive(Debug)]
pub struct Tokens {
    token: Vec<Token>,    // Token's vector
    idx: usize,           // Current index
    foremost: bool,       // Foremost
    length: usize,        // Token length
}

//pub type Token = (TokenKind, u32);

#[allow(dead_code)]
impl Tokens {
    pub fn new() -> Self {
        let token: Vec<Token> = Vec::new();
        Tokens { token, idx: 0, foremost: true, length: 0 }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, kind: TokenKind, line: u32) {
        self.length += 1;
        self.token.push(Token { kind, line });
    }

    pub fn reset(&mut self) {
        self.foremost = true;
        self.idx = 0;
    }

    pub fn consume(&mut self) -> Option<Token> {
        if self.foremost {
            self.foremost = false;
            Some(self.token[0].clone())
        } else {
            if self.idx+1 < self.length {
                self.idx += 1;
                Some(self.token[self.idx].clone())
            } else {
                None
            }
        }
    }

    pub fn get_token(&self) -> Token {
        self.token[self.idx].clone()
    }

    pub fn goto(&mut self, idx: usize) {
        self.idx = idx;
    }

    pub fn idx(&self) -> usize {
        self.idx
    }

    /// Get index of String same as TokenKind::ADDRESS() from TokenKind::LABEL()
    pub fn expect_address(&self) -> Result<usize, String> {
        if let TokenKind::ADDRESS(s) = self.token[self.idx].clone().kind {
            for t in &self.token {
                if let TokenKind::LABEL(name, idx) = &t.kind {
                    if &*s == &*name {
                        return Ok(*idx);
                    }
                }
            }
            let line = self.token[self.idx].line;
            Err(format!("{}: invalid address: {}", line, s))
        } else {
            let t = self.token[self.idx].clone();
            Err(format!("{}: expect TokenKind::ADDRESS(String). but got: {:?}", t.line, t.kind))
        }
    }

    pub fn expect_instruction(&self) -> Result<InstructionKind, String> {
        if let TokenKind::INSTRUCTION(k) = self.token[self.idx].kind {
            Ok(k)
        } else {
            let t = self.token[self.idx].clone();
            Err(format!("{}: expect TokenKind::INSTRUCTION(InstructionKind). but got: {:?}", t.line, t.kind))
        }
    }

    pub fn expect_register(&self) -> Result<usize, String> {
        if let TokenKind::REGISTER(_, i) = self.token[self.idx].kind {
            Ok(i)
        } else {
            let t = self.token[self.idx].clone();
            Err(format!("{}: expect TokenKind::REGISTER(RegisterKind, usize). but got: {:?}", t.line, t.kind))
        }
    }

    /// Return: Ok((register_idx, append idx))
    pub fn expect_stack(&self) -> Result<(usize, i32), String> {
        if let TokenKind::STACK(_, i, j) = self.token[self.idx].kind {
            Ok((i, j))
        } else {
            let t = self.token[self.idx].clone();
            Err(format!("{}: expect TokenKind::STACK(RegisterKind, usize, usize). but got: {:?}", t.line, t.kind))
        }
    }

    pub fn expect_integer(&self) -> Result<i32, String> {
        if let TokenKind::INTEGER(i) = self.token[self.idx].kind {
            Ok(i)
        } else {
            let t = self.token[self.idx].clone();
            Err(format!("{}: expect TokenKind::INTEGER(i32). but got: {:?}", t.line, t.kind))
        }
    }

    pub fn expect_eol(&self) -> Result<(), String> {
        if let TokenKind::EOL = self.token[self.idx].kind {
            Ok(())
        } else {
            let t = self.token[self.idx].clone();
            Err(format!("{}: expect TokenKind::EOL. but got: {:?}", t.line, t.kind))
        }
    }
}

