mod token;
mod lexer;
mod parser;

use std::env;
use std::io::{BufRead, BufReader};

use token::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Invalid argument");
    }

    let mut tokens: Tokens = Tokens::new();
    let mut reader = BufReader::new(std::fs::File::open(&args[1])
        .expect("Failed file open"));
    let mut buf = String::new();
    let mut number_of_lines: u32 = 1;
    while reader.read_line(&mut buf).unwrap() > 0 {
        lexer::tokenize(number_of_lines, &buf, &mut tokens);
        number_of_lines += 1;
        buf.clear();
    }

    let ra = parser::parse(tokens);

    std::process::exit(ra);
}

