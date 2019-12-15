
#[test]
#[cfg(test)]
fn test_parse() {
    use super::*;

    let mut tokens: Tokens = Tokens::new();

    tokens.push(TokenKind::INSTRUCTION(InstructionKind::ADDI), 1);
    tokens.push(TokenKind::REGISTER(RegisterKind::t0, 8), 2);
    tokens.push(TokenKind::REGISTER(RegisterKind::t0, 8), 3);
    tokens.push(TokenKind::INTEGER(1), 4);
    tokens.push(TokenKind::EOL, 5);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::ADD), 6);
    tokens.push(TokenKind::REGISTER(RegisterKind::t1,  9), 7);
    tokens.push(TokenKind::REGISTER(RegisterKind::t2, 10), 8);
    tokens.push(TokenKind::REGISTER(RegisterKind::t3, 11), 9);
    tokens.push(TokenKind::EOL, 10);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::SUB), 11);
    tokens.push(TokenKind::REGISTER(RegisterKind::t4, 12), 12);
    tokens.push(TokenKind::REGISTER(RegisterKind::t5, 13), 13);
    tokens.push(TokenKind::REGISTER(RegisterKind::t6, 14), 14);
    tokens.push(TokenKind::EOL, 15);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::XOR), 16);
    tokens.push(TokenKind::REGISTER(RegisterKind::t1, 9), 17);
    tokens.push(TokenKind::REGISTER(RegisterKind::t1, 9), 18);
    tokens.push(TokenKind::REGISTER(RegisterKind::t1, 9), 19);
    tokens.push(TokenKind::EOL, 20);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::LI), 21);
    tokens.push(TokenKind::REGISTER(RegisterKind::v0, 2), 22);
    tokens.push(TokenKind::INTEGER(1), 23);
    tokens.push(TokenKind::EOL, 24);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::MOVE), 25);
    tokens.push(TokenKind::REGISTER(RegisterKind::a0,  4), 26);
    tokens.push(TokenKind::REGISTER(RegisterKind::t2, 10), 27);
    tokens.push(TokenKind::EOL, 28);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::SYSCALL), 29);
    tokens.push(TokenKind::EOL, 30);
    tokens.push(TokenKind::LABEL("loop".to_string(), 30, None), 31);
    tokens.push(TokenKind::EOL, 32);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::ADDI), 33);
    tokens.push(TokenKind::REGISTER(RegisterKind::t0, 8), 34);
    tokens.push(TokenKind::REGISTER(RegisterKind::t0, 8), 35);
    tokens.push(TokenKind::INTEGER(1), 36);
    tokens.push(TokenKind::EOL, 37);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::BLT), 38);
    tokens.push(TokenKind::REGISTER(RegisterKind::t0, 8), 39);
    tokens.push(TokenKind::REGISTER(RegisterKind::t1, 9), 40);
    tokens.push(TokenKind::ADDRESS("loop".to_string()), 41);
    tokens.push(TokenKind::EOL, 42);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::MUL), 43);
    tokens.push(TokenKind::REGISTER(RegisterKind::t4, 12), 44);
    tokens.push(TokenKind::REGISTER(RegisterKind::t5, 13), 45);
    tokens.push(TokenKind::REGISTER(RegisterKind::t6, 14), 46);
    tokens.push(TokenKind::EOL, 47);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::J), 48);
    tokens.push(TokenKind::ADDRESS("hoge".to_string()), 49);
    tokens.push(TokenKind::EOL, 50);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::JAL), 51);
    tokens.push(TokenKind::ADDRESS("fuga".to_string()), 52);
    tokens.push(TokenKind::EOL, 53);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::SLT), 54);
    tokens.push(TokenKind::REGISTER(RegisterKind::t0,  8), 55);
    tokens.push(TokenKind::REGISTER(RegisterKind::t7, 15), 56);
    tokens.push(TokenKind::REGISTER(RegisterKind::v0,  2), 57);
    tokens.push(TokenKind::EOL, 58);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::NOT), 59);
    tokens.push(TokenKind::REGISTER(RegisterKind::t7, 15), 60);
    tokens.push(TokenKind::REGISTER(RegisterKind::v0,  2), 61);
    tokens.push(TokenKind::EOL, 62);

    let mut registers: [i32; 32] = [0; 32];
    let mut hi: u32 = 0;
    let mut lo: u32 = 0;
    let mut data:  Vec<u8> = Vec::new();
    let mut stack: Vec<u8> = vec![0];

    parse(&mut tokens, &mut registers, &mut hi, &mut lo, &mut data, &mut stack)
        .unwrap();
}

