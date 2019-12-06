use super::token::*;
use std::collections::VecDeque;

fn is_register(word: &&str) -> Result<(RegisterKind, usize), String> {
    if word.as_bytes()[0] != b'$' {
        return Err(format!("Invalid register name: {}", word));
    }

    let mut identifier = word.to_string();
    identifier.remove(0);  // Delete '$'

    let (register_kind, idx) = match &*identifier {
        "zero" |  "0" => (RegisterKind::zero, 0),
        "at"   |  "1" => (RegisterKind::at,   1),
        "v0"   |  "2" => (RegisterKind::v0,   2),
        "v1"   |  "3" => (RegisterKind::v1,   3),
        "a0"   |  "4" => (RegisterKind::a0,   4),
        "a1"   |  "5" => (RegisterKind::a1,   5),
        "a2"   |  "6" => (RegisterKind::a2,   6),
        "a3"   |  "7" => (RegisterKind::a3,   7),
        "t0"   |  "8" => (RegisterKind::t0,   8),
        "t1"   |  "9" => (RegisterKind::t1,   9),
        "t2"   | "10" => (RegisterKind::t2,  10),
        "t3"   | "11" => (RegisterKind::t3,  11),
        "t4"   | "12" => (RegisterKind::t4,  12),
        "t5"   | "13" => (RegisterKind::t5,  13),
        "t6"   | "14" => (RegisterKind::t6,  14),
        "t7"   | "15" => (RegisterKind::t7,  15),
        "s0"   | "16" => (RegisterKind::s0,  16),
        "s1"   | "17" => (RegisterKind::s1,  17),
        "s2"   | "18" => (RegisterKind::s2,  18),
        "s3"   | "19" => (RegisterKind::s3,  19),
        "s4"   | "20" => (RegisterKind::s4,  20),
        "s5"   | "21" => (RegisterKind::s5,  21),
        "s6"   | "22" => (RegisterKind::s6,  22),
        "s7"   | "23" => (RegisterKind::s7,  23),
        "t8"   | "24" => (RegisterKind::t8,  24),
        "t9"   | "25" => (RegisterKind::t9,  25),
        "k0"   | "26" => (RegisterKind::k0,  26),
        "k1"   | "27" => (RegisterKind::k1,  27),
        "gp"   | "28" => (RegisterKind::gp,  28),
        "sp"   | "29" => (RegisterKind::sp,  29),
        "fp"   | "30" => (RegisterKind::fp,  30),
        "ra"   | "31" => (RegisterKind::ra,  31),
        _ => return Err(format!("Invalid register name: {}", word)),
    };

    Ok((register_kind, idx))
}

pub fn tokenize(number_of_lines: u32, line: &str, tokens: &mut VecDeque<Token>) {
    let line = line.replace(",", " ");
    let words: Vec<&str> = line.split_whitespace().collect();

    // Skip blank line either comment line
    if words.len() == 0 || words.len() > 0 && words[0] == "#" {
        return;
    }

    for word in words {
        if let Ok(num) = word.parse::<i32>() {
            let t = Token::new(TokenKind::INTEGER(num), number_of_lines);
            tokens.push_back(t);
        } else if let Ok((k, i)) = is_register(&word) {
            let t = Token::new(TokenKind::REGISTER(k, i), number_of_lines);
            tokens.push_back(t);
        } else {
            let t = match &*word.to_ascii_uppercase() {
                // Arithmetic, Logic
                "ADD"  => Token::new(TokenKind::INSTRUCTION(InstructionKind::ADD),  number_of_lines),
                "ADDI" => Token::new(TokenKind::INSTRUCTION(InstructionKind::ADDI), number_of_lines),
                "SUB"  => Token::new(TokenKind::INSTRUCTION(InstructionKind::SUB),  number_of_lines),
                "XOR"  => Token::new(TokenKind::INSTRUCTION(InstructionKind::XOR),  number_of_lines),
                // Constant
                "LI"   => Token::new(TokenKind::INSTRUCTION(InstructionKind::LI),   number_of_lines),
                // Comparison
                // Branch, Jump
                // Load, Store
                // Transfer
                "MOVE" => Token::new(TokenKind::INSTRUCTION(InstructionKind::MOVE), number_of_lines),
                // Exception, Interrupt
                "SYSCALL" => Token::new(TokenKind::INSTRUCTION(InstructionKind::SYSCALL), number_of_lines),
                "#"    => break,
                _ => panic!("{}: invalid token: {}", number_of_lines, word),
            };
            tokens.push_back(t);
        }
    }

    tokens.push_back(Token::new(TokenKind::EOL, number_of_lines));
}
