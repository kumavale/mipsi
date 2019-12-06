#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InstructionKind {
    /// Arithmetic, Logic
    ADD,
    ADDI,
    SUB,
    XOR,

    /// Constant
    LI,

    /// Comparison

    /// Branch, Jump

    /// Load, Store

    /// Transfer
    MOVE,

    /// Exception, Interrupt
    SYSCALL,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    INSTRUCTION(InstructionKind),
    INTEGER(i32),
    REGISTER(RegisterKind, usize),  // (_, index)
    EOL,                            // End of Line
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub line: u32,        // Number of lines
}

impl Token {
    pub fn new(kind: TokenKind, line: u32) -> Self {
        Token { kind, line }
    }

    pub fn expect_instruction(&self) -> Result<InstructionKind, String> {
        if let TokenKind::INSTRUCTION(k) = self.kind {
            Ok(k)
        } else {
            //panic!("{}: expect TokenKind::INSTRUCTION(InstructionKind). but got: {:?}",
            //    self.line, self.kind);
            Err(format!("{}: expect TokenKind::INSTRUCTION(InstructionKind). but got: {:?}",
                self.line, self.kind))
        }
    }

    pub fn expect_register(&self) -> Result<usize, String> {
        if let TokenKind::REGISTER(_, i) = self.kind {
            Ok(i)
        } else {
            //panic!("{}: expect TokenKind::REGISTER(RegisterKind, usize). but got: {:?}",
            //    self.line, self.kind);
            Err(format!("{}: expect TokenKind::REGISTER(RegisterKind, usize). but got: {:?}",
                self.line, self.kind))
        }
    }

    pub fn expect_integer(&self) -> Result<i32, String> {
        if let TokenKind::INTEGER(i) = self.kind {
            Ok(i)
        } else {
            //panic!("{}: expect TokenKind::INTEGER(i32). but got: {:?}",
            //    );
            Err(format!("{}: expect TokenKind::INTEGER(i32). but got: {:?}",
                self.line, self.kind))
        }
    }

    pub fn expect_eol(&self) -> Result<(), String> {
        if TokenKind::EOL == self.kind {
            Ok(())
        } else {
            //panic!("{}: expect TokenKind::EOL. but got: {:?}",
            //    self.line, self.kind);
            Err(format!("{}: expect TokenKind::EOL. but got: {:?}",
                self.line, self.kind))
        }
    }
}

