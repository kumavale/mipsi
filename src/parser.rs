use super::token::*;
use std::collections::VecDeque;

pub fn parse(mut tokens: VecDeque<Token>) {

    let mut registers = [0; 32];

    while let Some(token) = tokens.pop_front() {
        //print!("{:?}", token);
        let instruction_kind = token.expect_instruction();

        match instruction_kind {
            InstructionKind::ADD  => {
                if let Some(token) = tokens.pop_front() {
                    let register_idx = token.expect_register();
                    registers[register_idx] = {
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
                        registers[r1_idx] + registers[r2_idx]
                    };
                    //println!("registers[{}]: {}", register_idx, registers[register_idx]);
                }
            },
            InstructionKind::ADDI => {
                if let Some(token) = tokens.pop_front() {
                    let register_idx = token.expect_register();
                    registers[register_idx] = {
                        let mut r1_idx = 0;
                        if let Some(token) = tokens.pop_front() {
                            let register_idx = token.expect_register();
                            r1_idx = register_idx;
                        }
                        let mut integer_literal = 0;
                        if let Some(token) = tokens.pop_front() {
                            integer_literal = token.expect_integer();
                        }
                        registers[r1_idx] + integer_literal
                    };
                    //println!("registers[{}]: {}", register_idx, registers[register_idx]);
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
                    registers[register_idx] = {
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
                        registers[r1_idx] ^ registers[r2_idx]
                    };
                    //println!("registers[{}]: {}", register_idx, registers[register_idx]);
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
    for (i, r) in registers.iter().enumerate() {
        println!("registers[{}]: {}", i, r);
    }

}
