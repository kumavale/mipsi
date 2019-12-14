mod repl;
mod token;
mod lexer;
mod parser;

use std::env;
use std::io::{BufRead, BufReader};

use token::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
    // REPL
        repl::run();

    } else {
    // 1 file
    // TODO multipl files
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

        let mut registers: [i32; 32] = [0; 32];
        let mut hi: u32 = 0;
        let mut lo: u32 = 0;
        let mut data:  Vec<u8> = Vec::new();
        let mut stack: Vec<u8> = vec![0];

        parser::parse(&mut tokens, &mut registers, &mut hi, &mut lo,
            &mut data, &mut stack);
    }
}

