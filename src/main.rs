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

