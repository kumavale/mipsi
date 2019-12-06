mod token;
mod lexer;

use std::env;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Invalid argument");
    }

    let mut tokens: Vec<token::Token> = Vec::new();
    let mut reader = BufReader::new(std::fs::File::open(&args[1])
        .expect("Failed file open"));
    let mut buf = String::new();
    while reader.read_line(&mut buf)? > 0 {
        lexer::tokenize(&buf, &mut tokens);
        buf.clear();
    }

    for token in tokens {
        print!("{:?}", token);
        if token.kind == token::TokenKind::EOL {
            println!("");
        }
    }

    Ok(())
}

#[test]
fn test_tokenize() {
    use token::*;

    let input = "\
# This is comment.
ADDI	r8,	r0,	1
ADD	r9,	r0,	r8
";

    let mut tokens: Vec<Token> = Vec::new();
    let mut buf = String::new();
    let mut reader = BufReader::new(input.as_bytes());
    while reader.read_line(&mut buf).unwrap() > 0 {
        lexer::tokenize(&buf, &mut tokens);
        buf.clear();
    }

    assert_eq!(tokens[0].kind, TokenKind::ADDI);
    assert_eq!(tokens[1].kind, TokenKind::REG("r8".to_string()));
    assert_eq!(tokens[2].kind, TokenKind::REG("r0".to_string()));
    assert_eq!(tokens[3].kind, TokenKind::INT(1));
    assert_eq!(tokens[4].kind, TokenKind::EOL);
    assert_eq!(tokens[5].kind, TokenKind::ADD);
    assert_eq!(tokens[6].kind, TokenKind::REG("r9".to_string()));
    assert_eq!(tokens[7].kind, TokenKind::REG("r0".to_string()));
    assert_eq!(tokens[8].kind, TokenKind::REG("r8".to_string()));
    assert_eq!(tokens[9].kind, TokenKind::EOL);

}

