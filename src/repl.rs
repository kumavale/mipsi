use std::io::{stdin, stdout, Write};

use super::token::Tokens;
use super::lexer::tokenize;
use super::parser::{parse, display::*};

pub fn run() {
    let mut tokens: Tokens = Tokens::new();
    let mut number_of_lines: u32 = 1;

    let mut registers: [i32; 32] = [0; 32];
    let mut hi: u32 = 0;
    let mut lo: u32 = 0;
    let mut data:  Vec<u8> = Vec::new();
    let mut stack: Vec<u8> = vec![0];

    println!("Welcome mipsi REPL!");
    println!("Type `exit` or ^C to exit");

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
            "DISPT"|"dispt" => { println!("{:?}", tokens.token); continue; }, // TODO provisional
            "DISPD"|"dispd" => { display_data_per_4byte(&data);  continue; },
            "DISPS"|"disps" => { display_stack(&stack);          continue; },
            "DISPR"|"dispr" => { display_register(&registers);   continue; },
            "" => continue,
            _ => (),
        }

        let old_tokens_len = tokens.len();
        tokenize(number_of_lines, 0, &input, &mut tokens);
        number_of_lines += 1;

        if 0 < tokens.len() {
            let result = parse(&mut tokens, &mut registers, &mut hi, &mut lo,
                &mut data, &mut stack);
            if let Err(e) = result {
                eprintln!("{}", e);
                // rollback
                let rollback_len = tokens.len() - old_tokens_len;
                for _ in 0..rollback_len {
                    tokens.pop();
                }
            }
        }

        println!();
    }
}

