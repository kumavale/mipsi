use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::PathBuf;

use super::token::Tokens;
use super::token::memory::Memory;
use super::lexer::tokenize;
use super::parser::parse;

pub fn run(files: &[PathBuf]) {
    let mut tokens: Tokens = Tokens::new();
    let mut memory = Memory::default();

    // Join files  =>  Everyone global
    for (filename_idx, file) in files.iter().enumerate() {
        let mut number_of_lines: u32 = 1;
        let mut reader = BufReader::new(File::open(file).expect("Failed file open"));
        tokens.add_file(file.to_str().unwrap());

        let mut buf = String::new();
        while 0 < reader.read_line(&mut buf).unwrap() {
            if let Err(e) = tokenize(number_of_lines, filename_idx, &buf, &mut tokens) {
                eprintln!("{}:{}: {}", file.display(), number_of_lines, e);
                std::process::exit(1);
            }
            number_of_lines += 1;
            buf.clear();
        }
    }

    // Execute
    if let Err(e) = parse(&mut tokens, &mut memory) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

