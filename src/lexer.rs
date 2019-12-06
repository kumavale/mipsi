use super::token::*;
use std::collections::VecDeque;

fn is_register(word: &&str) -> Result<(RegisterKind, usize), String> {
    let word_bytes = word.as_bytes();
    let register_kind = match word_bytes[0] {
        b'r' => RegisterKind::r,
        _  => return Err(format!("Invalid register name: {}", word)),
    };
    let mut i = 1;
    let mut idx = 0;
    while i < word_bytes.len() {
        idx = idx * 10 + (word_bytes[i] - '0' as u8) as usize;
        i += 1;
    }

    Ok((register_kind, idx))
}

pub fn tokenize(number_of_lines: u32, line: &str, tokens: &mut VecDeque<Token>) {
    let line = line.replace(",", " ");
    let words: Vec<&str> = line.split_whitespace().collect();
    let is_ignore_line = |t: &Vec<&str>| {
        t.len() == 0 || (t.len() > 0 && t[0] == "#")
    };

    if is_ignore_line(&words) {
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
            let t = match word {
                "ADD"  => Token::new(TokenKind::INSTRUCTION(InstructionKind::ADD),  number_of_lines),
                "ADDI" => Token::new(TokenKind::INSTRUCTION(InstructionKind::ADDI), number_of_lines),
                "SUB"  => Token::new(TokenKind::INSTRUCTION(InstructionKind::SUB),  number_of_lines),
                "XOR"  => Token::new(TokenKind::INSTRUCTION(InstructionKind::XOR),  number_of_lines),
                _ => panic!("Invalid token: \"{}\"", word),
            };
            tokens.push_back(t);
        }
    }

    tokens.push_back(Token::new(TokenKind::EOL, number_of_lines));
}
