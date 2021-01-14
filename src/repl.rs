use std::io::{stdin, stdout, Write};

use super::token::Tokens;
use super::token::memory::Memory;
use super::lexer::tokenize;
use super::parser::{parse, display::*};

pub fn run() {
    let mut tokens: Tokens = Tokens::new();
    let mut number_of_lines: u32 = 0;
    let mut memory = Memory::default();

    println!("Welcome mipsi REPL!");
    println!("Type `exit` or ^C to exit");
    println!("Type `help` to display help message\n");

    loop {
        print!("> ");
        let _ = stdout().flush();

        let input = {
            let mut s = String::new();
            stdin().read_line(&mut s).unwrap();
            s.trim_start().trim_end().to_owned()
        };

        match &*input {
            "exit"  => break,
            "help"  => { display_help();                                continue; },
            "dispt" => { println!("{:?}", tokens.token);                continue; }, // TODO provisional
            "dispd" => { display_data_per_4byte(&memory.static_data()); continue; },
            "disps" => { display_stack(&memory.stack());                continue; },
            "dispr" => { display_register(&memory.registers);           continue; },
            "dispf" => { display_fp_register(&memory.registers);        continue; },
            "" => continue,
            _ => (),
        }

        number_of_lines += 1;
        let old_tokens_len = tokens.len();

        if let Err(e) = tokenize(number_of_lines, 0, &input, &mut tokens) {
            eprintln!("tokenize failed: {}\n", e);
            rollback(&mut tokens, old_tokens_len);
            continue;
        }

        if 0 < tokens.len() {
            let result = parse(&mut tokens, &mut memory);
            if let Err(e) = result {
                eprintln!("{}\n", e);
                rollback(&mut tokens, old_tokens_len);
                continue;
            }
        }

        println!();
    }
}

fn rollback(tokens: &mut Tokens, old_tokens_len: usize) {
    let rollback_len = tokens.len() - old_tokens_len;
    for _ in 0..rollback_len {
        tokens.pop();
    }
    tokens.back_idx();
}

