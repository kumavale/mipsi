#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InstructionKind {
    ADD,
    ADDI,
    SUB,
    XOR,
}

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum RegisterKind {
    r,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    INSTRUCTION(InstructionKind),
    INTEGER(i32),
    REGISTER(RegisterKind, usize),
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

    pub fn expect_instruction(&self) -> InstructionKind {
        if let TokenKind::INSTRUCTION(k) = self.kind {
            return k;
        } else {
            panic!("{}: expect TokenKind::INSTRUCTION(InstructionKind). but got: {:?}",
                self.line, self.kind);
        }
    }

    pub fn expect_register(&self) -> usize {
        if let TokenKind::REGISTER(_, i) = self.kind {
            i
        } else {
            panic!("{}: expect TokenKind::REGISTER(String). but got: {:?}",
                self.line, self.kind);
        }
    }

    pub fn expect_integer(&self) -> i32 {
        if let TokenKind::INTEGER(i) = self.kind {
            i
        } else {
            panic!("{}: expect TokenKind::INTEGER(i32). but got: {:?}",
                self.line, self.kind);
        }
    }

    pub fn expect_eol(&self) {
        if TokenKind::EOL == self.kind {
            // Do nothing
        } else {
            panic!("{}: expect TokenKind::EOL. but got: {:?}",
                self.line, self.kind);
        }
    }
}

