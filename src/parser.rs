use super::token::*;
use std::collections::VecDeque;

pub fn parse(mut tokens: VecDeque<Token>) {

    let mut r = [0; 16];

    while let Some(token) = tokens.pop_front() {
        //print!("{:?}", token);
        let instruction_kind = token.expect_instruction();

        match instruction_kind {
            InstructionKind::ADD  => {
                if let Some(token) = tokens.pop_front() {
                    let register_idx = token.expect_register();
                    r[register_idx] = {
                        let mut r1_idx = 0;
                        if let Some(token) = tokens.pop_front() {
                            let register_idx = token.expect_register();
                            r1_idx = register_idx;
                        }
                        let mut r2_idx = 0;
                        if let Some(token) = tokens.pop_front() {
                            let register_idx = token.expect_register();
                            r2_idx = register_idx;
                        }
                        r[r1_idx] + r[r2_idx]
                    };
                    //println!("r[{}]: {}", register_idx, r[register_idx]);
                }
            },
            InstructionKind::ADDI => {
                if let Some(token) = tokens.pop_front() {
                    let register_idx = token.expect_register();
                    r[register_idx] = {
                        let mut r1_idx = 0;
                        if let Some(token) = tokens.pop_front() {
                            let register_idx = token.expect_register();
                            r1_idx = register_idx;
                        }
                        let mut integer_literal = 0;
                        if let Some(token) = tokens.pop_front() {
                            integer_literal = token.expect_integer();
                        }
                        r[r1_idx] + integer_literal
                    };
                    //println!("r[{}]: {}", register_idx, r[register_idx]);
                }
            },
            InstructionKind::SUB  => {
                // TODO
                tokens.pop_front();
                tokens.pop_front();
                tokens.pop_front();
            },
            InstructionKind::XOR  => {
                if let Some(token) = tokens.pop_front() {
                    let register_idx = token.expect_register();
                    r[register_idx] = {
                        let mut r1_idx = 0;
                        if let Some(token) = tokens.pop_front() {
                            let register_idx = token.expect_register();
                            r1_idx = register_idx;
                        }
                        let mut r2_idx = 0;
                        if let Some(token) = tokens.pop_front() {
                            let register_idx = token.expect_register();
                            r2_idx = register_idx;
                        }
                        r[r1_idx] ^ r[r2_idx]
                    };
                    //println!("r[{}]: {}", register_idx, r[register_idx]);
                }
            },
            //_ => (),
        }

        // expect TokenKind::EOL
        if let Some(token) = tokens.pop_front() {
            token.expect_eol();
        }
    }

    // Display all registers
    for (i, r) in r.iter().enumerate() {
        println!("r[{}]: {}", i, r);
    }

}
