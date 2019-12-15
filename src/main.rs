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
        // Join files  =>  Everyone global
        let mut tokens: Tokens = Tokens::new();
        let mut number_of_lines: u32 = 1;

        for file in args.iter().skip(1) {
            let mut reader = BufReader::new(std::fs::File::open(file)
                .expect("Failed file open"));
                let mut buf = String::new();
                while reader.read_line(&mut buf).unwrap() > 0 {
                    lexer::tokenize(number_of_lines, &buf, &mut tokens);
                    number_of_lines += 1;
                    buf.clear();
                }
        }

        let mut registers: [i32; 32] = [0; 32];
        let mut hi: u32 = 0;
        let mut lo: u32 = 0;
        let mut data:  Vec<u8> = Vec::new();
        let mut stack: Vec<u8> = vec![0];

        parser::parse(&mut tokens, &mut registers, &mut hi, &mut lo,
            &mut data, &mut stack)
            .unwrap();
    }
}

