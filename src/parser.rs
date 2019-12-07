use super::token::*;
use std::collections::VecDeque;

pub fn parse(mut tokens: VecDeque<Token>) {

    let mut registers = [0; 32];
    //let mut data: Vec<String> = vec![];

    while let Some(token) = tokens.pop_front() {
        //print!("{:?}", token);
        //if let == TokenKind::ADDRESS(String)
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
                match registers[2] {                                 // v0
                    // print_int
                    1 => print!("{}", registers[4]),                 // a0
                    // print_string
                    //4 => print!("{}", data[registers[4] as usize]),  // a0
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

#[test]
#[cfg(test)]
fn test_parse() {

    let mut tokens: VecDeque<Token> = VecDeque::new();

    tokens.push_back(Token::new(TokenKind::INSTRUCTION(InstructionKind::ADDI), 1));
    tokens.push_back(Token::new(TokenKind::REGISTER(RegisterKind::t0, 8), 2));
    tokens.push_back(Token::new(TokenKind::REGISTER(RegisterKind::t0, 8), 3));
    tokens.push_back(Token::new(TokenKind::INTEGER(1), 4));
    tokens.push_back(Token::new(TokenKind::EOL, 5));
    tokens.push_back(Token::new(TokenKind::INSTRUCTION(InstructionKind::ADD), 6));
    tokens.push_back(Token::new(TokenKind::REGISTER(RegisterKind::t1,  9), 7));
    tokens.push_back(Token::new(TokenKind::REGISTER(RegisterKind::t2, 10), 8));
    tokens.push_back(Token::new(TokenKind::REGISTER(RegisterKind::t3, 11), 9));
    tokens.push_back(Token::new(TokenKind::EOL, 10));
    tokens.push_back(Token::new(TokenKind::INSTRUCTION(InstructionKind::SUB), 11));
    tokens.push_back(Token::new(TokenKind::REGISTER(RegisterKind::t4, 12), 12));
    tokens.push_back(Token::new(TokenKind::REGISTER(RegisterKind::t5, 13), 13));
    tokens.push_back(Token::new(TokenKind::REGISTER(RegisterKind::t6, 14), 14));
    tokens.push_back(Token::new(TokenKind::EOL, 15));
    tokens.push_back(Token::new(TokenKind::INSTRUCTION(InstructionKind::XOR), 16));
    tokens.push_back(Token::new(TokenKind::REGISTER(RegisterKind::t1, 9), 17));
    tokens.push_back(Token::new(TokenKind::REGISTER(RegisterKind::t1, 9), 18));
    tokens.push_back(Token::new(TokenKind::REGISTER(RegisterKind::t1, 9), 19));
    tokens.push_back(Token::new(TokenKind::EOL, 20));
    tokens.push_back(Token::new(TokenKind::INSTRUCTION(InstructionKind::LI), 21));
    tokens.push_back(Token::new(TokenKind::REGISTER(RegisterKind::v0, 2), 22));
    tokens.push_back(Token::new(TokenKind::INTEGER(1), 23));
    tokens.push_back(Token::new(TokenKind::EOL, 24));
    tokens.push_back(Token::new(TokenKind::INSTRUCTION(InstructionKind::MOVE), 25));
    tokens.push_back(Token::new(TokenKind::REGISTER(RegisterKind::a0,  4), 26));
    tokens.push_back(Token::new(TokenKind::REGISTER(RegisterKind::t2, 10), 27));
    tokens.push_back(Token::new(TokenKind::EOL, 28));
    tokens.push_back(Token::new(TokenKind::INSTRUCTION(InstructionKind::SYSCALL), 29));
    tokens.push_back(Token::new(TokenKind::EOL, 30));

    parse(tokens);
}

