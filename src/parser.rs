use super::token::*;

pub fn parse(mut tokens: Tokens) {

    let mut registers = [0; 32];
    //let mut data: Vec<String> = vec![];

    while let Some(token) = tokens.consume() {
        //println!("{:?}", token);

        if let TokenKind::LABEL(_, _) = token.0 {
            tokens.consume().unwrap();
            if let Ok(_) = tokens.expect_eol() {
                continue;
            }
        }

        let instruction_kind = tokens.expect_instruction().unwrap();

        match instruction_kind {
            // Arithmetic, Logic
            InstructionKind::ADD |
            InstructionKind::ADDI =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| x + y),
            InstructionKind::SUB =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| x - y),
            InstructionKind::MUL =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| x * y),
            InstructionKind::DIV =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| x / y),
            InstructionKind::AND |
            InstructionKind::ANDI =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| x & y),
            InstructionKind::OR |
            InstructionKind::ORI =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| x | y),
            InstructionKind::XOR |
            InstructionKind::XORI =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| x ^ y),

            // Constant
            InstructionKind::LI =>
                eval_constant(&mut registers, &mut tokens, |x| x),
            InstructionKind::LUI =>
                eval_constant(&mut registers, &mut tokens, |x| x & (std::i32::MAX - 65535)),

            // Comparison

            // Branch
            InstructionKind::BLE => {
            },
            InstructionKind::BLT => {
                if let Some(_) = tokens.consume() {
                    if let Ok(rsrc1_idx) = tokens.expect_register() {
                        if let Some(_) = tokens.consume() {
                            if let Ok(rsrc2_idx) = tokens.expect_register() {
                                if let Some(_) = tokens.consume() {
                                    if registers[rsrc1_idx] < registers[rsrc2_idx] {
                                        let idx = tokens.expect_address().unwrap();
                                        tokens.goto(idx-1);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            },

            // Jump
            InstructionKind::J =>
                if eval_jump(&mut registers, &mut tokens, InstructionKind::J)    { continue; },
            InstructionKind::JAL =>
                if eval_jump(&mut registers, &mut tokens, InstructionKind::JAL)  { continue; },
            InstructionKind::JR =>
                if eval_jump(&mut registers, &mut tokens, InstructionKind::JR)   { continue; },
            InstructionKind::JALR =>
                if eval_jump(&mut registers, &mut tokens, InstructionKind::JALR) { continue; },

            // Load, Store
            InstructionKind::LA => {
            },
            InstructionKind::LW => {
            },
            InstructionKind::SW => {
            },

            // Transfer
            InstructionKind::MOVE => {
                if let Some(_) = tokens.consume() {
                    let register_idx = tokens.expect_register().unwrap();
                    registers[register_idx] = {
                        let mut r1_idx = 0;
                        if let Some(_) = tokens.consume() {
                            let register_idx = tokens.expect_register().unwrap();
                            r1_idx = register_idx;
                        }
                        registers[r1_idx]
                    };
                }
            },

            // Exception, Interrupt
            InstructionKind::SYSCALL => {
                match registers[2] {                                 // v0
                    // print_int
                    1  => print!("{}", registers[4]),                 // a0
                    // print_string
                    //4  => print!("{}", data[registers[4] as usize]),  // a0
                    // exit
                    10 => break,
                    _ => (),
                }
            },
            InstructionKind::NOP => (),  // Do nothing
            //_ => (),
        }

        // expect TokenKind::EOL
        tokens.consume();
        tokens.expect_eol().unwrap();
    }

    // Display all registers
    // `DEBUG=1 cargo run`
    if std::env::var("DEBUG").is_ok() {
        println!("\n[Display all registers]");
        for i in 0..8 {
            for j in 0..4 {
                print!("${:<2}:0x{:08x}\t", i+j*8, registers[i+j*8]);
            }
            println!("");
        }
    }
}

fn eval_arithmetic<F>(registers: &mut [i32], tokens: &mut Tokens, fun: F)
where
    F: Fn(i32, i32) -> i32,
{
    if let Some(_) = tokens.consume() {
        if let Ok(rd_idx) = tokens.expect_register() {
            registers[rd_idx] = {
                let mut r1 = 0;
                if let Some(_) = tokens.consume() {
                    if let Ok(register_idx) = tokens.expect_register() {
                        r1 = registers[register_idx];
                    } else if let Ok(num) = tokens.expect_integer() {
                        r1 = num;
                    }
                }
                let mut r2 = 0;
                if let Some(_) = tokens.consume() {
                    if let Ok(register_idx) = tokens.expect_register() {
                        r2 = registers[register_idx];
                    } else if let Ok(num) = tokens.expect_integer() {
                        r2 = num;
                    }
                }
                fun(r1, r2)
            };
        }
    }
}

fn eval_constant<F>(registers: &mut [i32], tokens: &mut Tokens, fun: F)
where
    F: Fn(i32) -> i32,
{
    if let Some(_) = tokens.consume() {
        let register_idx = tokens.expect_register().unwrap();
        registers[register_idx] = {
            let mut integer = 0;
            if let Some(_) = tokens.consume() {
                integer = tokens.expect_integer().unwrap();
            }
            fun(integer)
        };
    }
}

/// Return: can continue
fn eval_jump(registers: &mut [i32], tokens: &mut Tokens, kind: InstructionKind) -> bool {
    match kind {
        InstructionKind::J =>
            if let Some(_) = tokens.consume() {
                if let Ok(idx) = tokens.expect_address() {
                    tokens.goto(idx-1);
                    return true;
                }
            },
        InstructionKind::JAL =>
            if let Some(_) = tokens.consume() {
                if let Ok(idx) = tokens.expect_address() {
                    registers[31] = tokens.idx() as i32 + 1;  // $ra
                    tokens.goto(idx-1);
                    return true;
                }
            },
        InstructionKind::JR =>
            if let Some(_) = tokens.consume() {
                if let Ok(idx) = tokens.expect_register() {
                    tokens.goto(registers[idx] as usize);
                    return true;
                }
            },
        InstructionKind::JALR =>
            if let Some(_) = tokens.consume() {
                if let Ok(rs_idx) = tokens.expect_register() {
                    tokens.consume();
                    if let Ok(rd_idx) = tokens.expect_register() {
                        registers[rd_idx] = tokens.idx() as i32 + 1;
                        tokens.goto(rs_idx-1);
                        return true;
                    }
                }
            },
        _ => panic!("eval_jump(): invalid InstructionKind: {:?}", kind),
    }

    false
}

#[test]
#[cfg(test)]
fn test_parse() {

    let mut tokens: Tokens = Tokens::new();

    tokens.push(TokenKind::INSTRUCTION(InstructionKind::ADDI), 1);
    tokens.push(TokenKind::REGISTER(RegisterKind::t0, 8), 2);
    tokens.push(TokenKind::REGISTER(RegisterKind::t0, 8), 3);
    tokens.push(TokenKind::INTEGER(1), 4);
    tokens.push(TokenKind::EOL, 5);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::ADD), 6);
    tokens.push(TokenKind::REGISTER(RegisterKind::t1,  9), 7);
    tokens.push(TokenKind::REGISTER(RegisterKind::t2, 10), 8);
    tokens.push(TokenKind::REGISTER(RegisterKind::t3, 11), 9);
    tokens.push(TokenKind::EOL, 10);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::SUB), 11);
    tokens.push(TokenKind::REGISTER(RegisterKind::t4, 12), 12);
    tokens.push(TokenKind::REGISTER(RegisterKind::t5, 13), 13);
    tokens.push(TokenKind::REGISTER(RegisterKind::t6, 14), 14);
    tokens.push(TokenKind::EOL, 15);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::XOR), 16);
    tokens.push(TokenKind::REGISTER(RegisterKind::t1, 9), 17);
    tokens.push(TokenKind::REGISTER(RegisterKind::t1, 9), 18);
    tokens.push(TokenKind::REGISTER(RegisterKind::t1, 9), 19);
    tokens.push(TokenKind::EOL, 20);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::LI), 21);
    tokens.push(TokenKind::REGISTER(RegisterKind::v0, 2), 22);
    tokens.push(TokenKind::INTEGER(1), 23);
    tokens.push(TokenKind::EOL, 24);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::MOVE), 25);
    tokens.push(TokenKind::REGISTER(RegisterKind::a0,  4), 26);
    tokens.push(TokenKind::REGISTER(RegisterKind::t2, 10), 27);
    tokens.push(TokenKind::EOL, 28);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::SYSCALL), 29);
    tokens.push(TokenKind::EOL, 30);
    tokens.push(TokenKind::LABEL("loop".to_string(), 30), 31);
    tokens.push(TokenKind::EOL, 32);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::ADDI), 33);
    tokens.push(TokenKind::REGISTER(RegisterKind::t0, 8), 34);
    tokens.push(TokenKind::REGISTER(RegisterKind::t0, 8), 35);
    tokens.push(TokenKind::INTEGER(1), 36);
    tokens.push(TokenKind::EOL, 37);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::BLT), 38);
    tokens.push(TokenKind::REGISTER(RegisterKind::t0, 8), 39);
    tokens.push(TokenKind::REGISTER(RegisterKind::t1, 9), 40);
    tokens.push(TokenKind::ADDRESS("loop".to_string()), 41);
    tokens.push(TokenKind::EOL, 42);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::MUL), 43);
    tokens.push(TokenKind::REGISTER(RegisterKind::t4, 12), 44);
    tokens.push(TokenKind::REGISTER(RegisterKind::t5, 13), 45);
    tokens.push(TokenKind::REGISTER(RegisterKind::t6, 14), 46);
    tokens.push(TokenKind::EOL, 47);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::J), 48);
    tokens.push(TokenKind::ADDRESS("hoge".to_string()), 49);
    tokens.push(TokenKind::EOL, 50);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::JAL), 51);
    tokens.push(TokenKind::ADDRESS("fuga".to_string()), 52);
    tokens.push(TokenKind::EOL, 53);

    parse(tokens);
}

