mod repl;
mod token;
mod lexer;
mod parser;

use std::env;
use std::io::{BufRead, BufReader};

use token::*;
use token::register::Registers;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        // REPL
        repl::run();

    } else {
        // Join files  =>  Everyone global
        let mut tokens: Tokens = Tokens::new();

        for (filename_idx, file) in args.iter().skip(1).enumerate() {
            let mut number_of_lines: u32 = 1;
            let mut reader = BufReader::new(std::fs::File::open(file)
                .expect("Failed file open"));
            tokens.add_file(&file);

            let mut buf = String::new();
            while reader.read_line(&mut buf).unwrap() > 0 {
                if let Err(e) = lexer::tokenize(number_of_lines, filename_idx, &buf, &mut tokens) {
                    panic!("tokenize failed: {}", e);
                }
                number_of_lines += 1;
                buf.clear();
            }
        }

        let mut registers = Registers::default();
        let mut hi: u32 = 0;
        let mut lo: u32 = 0;
        let mut data:  Vec<u8> = Vec::new();
        let mut stack: Vec<u8> = vec![0];

        parser::parse(&mut tokens, &mut registers, &mut hi, &mut lo,
            &mut data, &mut stack)
            .unwrap();

        //println!("{:?}", tokens);
        //parser::display::display_data_per_4byte(&data);
        //parser::display::display_stack(&stack);
        //parser::display::display_register(&registers);
    }
}

