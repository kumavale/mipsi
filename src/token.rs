#[derive(Debug, PartialEq)]
pub enum TokenKind {
    ADD,
    ADDI,
    SUB,
    XOR,
    INT(i32),
    REG(String),
    EOL,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
}

impl Token {
    pub fn new(kind: TokenKind) -> Self {
        Token { kind }
    }
}

