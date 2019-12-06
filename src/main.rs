mod token;
mod lexer;
mod parser;

use std::env;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Invalid argument");
    }

    let mut tokens: VecDeque<token::Token> = VecDeque::new();
    let mut reader = BufReader::new(std::fs::File::open(&args[1])
        .expect("Failed file open"));
    let mut buf = String::new();
    let mut number_of_lines: u32 = 1;
    while reader.read_line(&mut buf)? > 0 {
        lexer::tokenize(number_of_lines, &buf, &mut tokens);
        number_of_lines += 1;
        buf.clear();
    }

    //for token in &tokens {
    //    print!("{:?}", token);
    //    if token.kind == TokenKind::EOL {
    //        println!("");
    //    }
    //}

    parser::parse(tokens);

    Ok(())
}

#[test]
#[cfg(test)]
fn test_tokenize() {
    use token::*;

    let input = "\
# This is comment.
ADDI	r8,	r0,	1
add	r9,	r0,	r8
SUB     r9,     r1,     r2
Xor     r3,     r4,     r5
";

    let mut tokens: VecDeque<token::Token> = VecDeque::new();
    let mut buf = String::new();
    let mut reader = BufReader::new(input.as_bytes());
    while reader.read_line(&mut buf).unwrap() > 0 {
        lexer::tokenize(0, &buf, &mut tokens);
        buf.clear();
    }

    assert_eq!(tokens[0].kind,  TokenKind::INSTRUCTION(InstructionKind::ADDI));
    assert_eq!(tokens[1].kind,  TokenKind::REGISTER(RegisterKind::r, 8));
    assert_eq!(tokens[2].kind,  TokenKind::REGISTER(RegisterKind::r, 0));
    assert_eq!(tokens[3].kind,  TokenKind::INTEGER(1));
    assert_eq!(tokens[4].kind,  TokenKind::EOL);
    assert_eq!(tokens[5].kind,  TokenKind::INSTRUCTION(InstructionKind::ADD));
    assert_eq!(tokens[6].kind,  TokenKind::REGISTER(RegisterKind::r, 9));
    assert_eq!(tokens[7].kind,  TokenKind::REGISTER(RegisterKind::r, 0));
    assert_eq!(tokens[8].kind,  TokenKind::REGISTER(RegisterKind::r, 8));
    assert_eq!(tokens[9].kind,  TokenKind::EOL);
    assert_eq!(tokens[10].kind, TokenKind::INSTRUCTION(InstructionKind::SUB));
    assert_eq!(tokens[11].kind, TokenKind::REGISTER(RegisterKind::r, 9));
    assert_eq!(tokens[12].kind, TokenKind::REGISTER(RegisterKind::r, 1));
    assert_eq!(tokens[13].kind, TokenKind::REGISTER(RegisterKind::r, 2));
    assert_eq!(tokens[14].kind, TokenKind::EOL);
    assert_eq!(tokens[15].kind, TokenKind::INSTRUCTION(InstructionKind::XOR));
    assert_eq!(tokens[16].kind, TokenKind::REGISTER(RegisterKind::r, 3));
    assert_eq!(tokens[17].kind, TokenKind::REGISTER(RegisterKind::r, 4));
    assert_eq!(tokens[18].kind, TokenKind::REGISTER(RegisterKind::r, 5));
    assert_eq!(tokens[19].kind, TokenKind::EOL);
}

#[test]
#[cfg(test)]
fn test_parse() {
    use token::*;

    let mut tokens: VecDeque<token::Token> = VecDeque::new();

    tokens.push_back(Token::new(TokenKind::INSTRUCTION(InstructionKind::ADDI), 1));
    tokens.push_back(Token::new(TokenKind::REGISTER(RegisterKind::r, 8), 2));
    tokens.push_back(Token::new(TokenKind::REGISTER(RegisterKind::r, 0), 3));
    tokens.push_back(Token::new(TokenKind::INTEGER(1), 4));
    tokens.push_back(Token::new(TokenKind::EOL, 5));
    tokens.push_back(Token::new(TokenKind::INSTRUCTION(InstructionKind::ADD), 6));
    tokens.push_back(Token::new(TokenKind::REGISTER(RegisterKind::r, 9), 7));
    tokens.push_back(Token::new(TokenKind::REGISTER(RegisterKind::r, 0), 8));
    tokens.push_back(Token::new(TokenKind::REGISTER(RegisterKind::r, 8), 9));
    tokens.push_back(Token::new(TokenKind::EOL, 10));
    tokens.push_back(Token::new(TokenKind::INSTRUCTION(InstructionKind::SUB), 11));
    tokens.push_back(Token::new(TokenKind::REGISTER(RegisterKind::r, 9), 12));
    tokens.push_back(Token::new(TokenKind::REGISTER(RegisterKind::r, 1), 13));
    tokens.push_back(Token::new(TokenKind::REGISTER(RegisterKind::r, 2), 14));
    tokens.push_back(Token::new(TokenKind::EOL, 15));
    tokens.push_back(Token::new(TokenKind::INSTRUCTION(InstructionKind::XOR), 16));
    tokens.push_back(Token::new(TokenKind::REGISTER(RegisterKind::r, 3), 17));
    tokens.push_back(Token::new(TokenKind::REGISTER(RegisterKind::r, 4), 18));
    tokens.push_back(Token::new(TokenKind::REGISTER(RegisterKind::r, 5), 19));
    tokens.push_back(Token::new(TokenKind::EOL, 20));

    parser::parse(tokens);
}

