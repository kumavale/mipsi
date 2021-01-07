
#[test]
#[cfg(test)]
fn test_parse() {
    use super::*;
    use crate::token::register::*;

    let mut tokens: Tokens = Tokens::new();

    tokens.push(TokenKind::INSTRUCTION(InstructionKind::ADDI), 1, 0);
    tokens.push(TokenKind::REGISTER(RegisterKind::t0, 8), 2, 0);
    tokens.push(TokenKind::REGISTER(RegisterKind::t0, 8), 3, 0);
    tokens.push(TokenKind::INTEGER(1), 4, 0);
    tokens.push(TokenKind::EOL, 5, 0);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::ADD), 6, 0);
    tokens.push(TokenKind::REGISTER(RegisterKind::t1,  9), 7, 0);
    tokens.push(TokenKind::REGISTER(RegisterKind::t2, 10), 8, 0);
    tokens.push(TokenKind::REGISTER(RegisterKind::t3, 11), 9, 0);
    tokens.push(TokenKind::EOL, 10, 0);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::SUB), 11, 0);
    tokens.push(TokenKind::REGISTER(RegisterKind::t4, 12), 12, 0);
    tokens.push(TokenKind::REGISTER(RegisterKind::t5, 13), 13, 0);
    tokens.push(TokenKind::REGISTER(RegisterKind::t6, 14), 14, 0);
    tokens.push(TokenKind::EOL, 15, 0);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::XOR), 16, 0);
    tokens.push(TokenKind::REGISTER(RegisterKind::t1, 9), 17, 0);
    tokens.push(TokenKind::REGISTER(RegisterKind::t1, 9), 18, 0);
    tokens.push(TokenKind::REGISTER(RegisterKind::t1, 9), 19, 0);
    tokens.push(TokenKind::EOL, 20, 0);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::LI), 21, 0);
    tokens.push(TokenKind::REGISTER(RegisterKind::v0, 2), 22, 0);
    tokens.push(TokenKind::INTEGER(1), 23, 0);
    tokens.push(TokenKind::EOL, 24, 0);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::MOVE), 25, 0);
    tokens.push(TokenKind::REGISTER(RegisterKind::a0,  4), 26, 0);
    tokens.push(TokenKind::REGISTER(RegisterKind::t2, 10), 27, 0);
    tokens.push(TokenKind::EOL, 28, 0);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::SYSCALL), 29, 0);
    tokens.push(TokenKind::EOL, 30, 0);
    tokens.push(TokenKind::LABEL("loop".to_string(), 30, None), 31, 0);
    tokens.push(TokenKind::EOL, 32, 0);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::ADDI), 33, 0);
    tokens.push(TokenKind::REGISTER(RegisterKind::t0, 8), 34, 0);
    tokens.push(TokenKind::REGISTER(RegisterKind::t0, 8), 35, 0);
    tokens.push(TokenKind::INTEGER(1), 36, 0);
    tokens.push(TokenKind::EOL, 37, 0);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::BLT), 38, 0);
    tokens.push(TokenKind::REGISTER(RegisterKind::t0, 8), 39, 0);
    tokens.push(TokenKind::REGISTER(RegisterKind::t1, 9), 40, 0);
    tokens.push(TokenKind::ADDRESS("loop".to_string()), 41, 0);
    tokens.push(TokenKind::EOL, 42, 0);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::MUL), 43, 0);
    tokens.push(TokenKind::REGISTER(RegisterKind::t4, 12), 44, 0);
    tokens.push(TokenKind::REGISTER(RegisterKind::t5, 13), 45, 0);
    tokens.push(TokenKind::REGISTER(RegisterKind::t6, 14), 46, 0);
    tokens.push(TokenKind::EOL, 47, 0);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::SLT), 48, 0);
    tokens.push(TokenKind::REGISTER(RegisterKind::t0,  8), 49, 0);
    tokens.push(TokenKind::REGISTER(RegisterKind::t7, 15), 50, 0);
    tokens.push(TokenKind::REGISTER(RegisterKind::v0,  2), 51, 0);
    tokens.push(TokenKind::EOL, 52, 0);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::NOT), 53, 0);
    tokens.push(TokenKind::REGISTER(RegisterKind::t7, 15), 54, 0);
    tokens.push(TokenKind::REGISTER(RegisterKind::v0,  2), 55, 0);
    tokens.push(TokenKind::EOL, 56, 0);

    let mut memory = Memory::default();

    parse(&mut tokens, &mut memory).unwrap();
}

