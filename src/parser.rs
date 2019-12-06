use super::token::*;
use std::collections::VecDeque;

pub fn parse(mut tokens: VecDeque<Token>) {

    let mut registers = [0; 32];

    while let Some(token) = tokens.pop_front() {
        //print!("{:?}", token);
        let instruction_kind = token.expect_instruction().unwrap();

        match instruction_kind {
            InstructionKind::ADD |
            InstructionKind::ADDI => {
                eval_arithmetic(&mut registers, &mut tokens, |x, y| x + y);
            },
            InstructionKind::SUB  => {
                eval_arithmetic(&mut registers, &mut tokens, |x, y| x - y);
            },
            InstructionKind::XOR  => {
                eval_arithmetic(&mut registers, &mut tokens, |x, y| x ^ y);
            },
            InstructionKind::LI  => {
                if let Some(token) = tokens.pop_front() {
                    let register_idx = token.expect_register().unwrap();
                    registers[register_idx] = {
                        let mut integer_literal = 0;
                        if let Some(token) = tokens.pop_front() {
                            integer_literal = token.expect_integer().unwrap();
                        }
                        integer_literal
                    };
                }
            },
            InstructionKind::MOVE  => {
                if let Some(token) = tokens.pop_front() {
                    let register_idx = token.expect_register().unwrap();
                    registers[register_idx] = {
                        let mut r1_idx = 0;
                        if let Some(token) = tokens.pop_front() {
                            let register_idx = token.expect_register().unwrap();
                            r1_idx = register_idx;
                        }
                        registers[r1_idx]
                    };
                }
            },
            InstructionKind::SYSCALL  => {
                //match register[v0] // TODO HashMap
                match registers[2] {
                    // print_int
                    1 => print!("{}", registers[4]),
                    _ => (),
                }
            },
            //_ => (),
        }

        // expect TokenKind::EOL
        if let Some(token) = tokens.pop_front() {
            if let Err(e) = token.expect_eol() {
                panic!("{}", e);
            }
        }
    }

    // Display all registers
    //for (i, r) in registers.iter().enumerate() {
    //    println!("registers[{}]: {}", i, r);
    //}

}

fn eval_arithmetic<F>(registers: &mut [i32], tokens: &mut VecDeque<Token>, fun: F)
where
    F: Fn(i32, i32) -> i32,
{
    if let Some(token) = tokens.pop_front() {
        if let Ok(rd_idx) = token.expect_register() {
            registers[rd_idx] = {
                let mut r1 = 0;
                if let Some(token) = tokens.pop_front() {
                    if let Ok(register_idx) = token.expect_register() {
                        r1 = registers[register_idx];
                    } else if let Ok(num) = token.expect_integer() {
                        r1 = num;
                    }
                }
                let mut r2 = 0;
                if let Some(token) = tokens.pop_front() {
                    if let Ok(register_idx) = token.expect_register() {
                        r2 = registers[register_idx];
                    } else if let Ok(num) = token.expect_integer() {
                        r2 = num;
                    }
                }
                fun(r1, r2)
            };
        }
    }
}

