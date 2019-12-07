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

