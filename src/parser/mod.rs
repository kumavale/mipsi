use std::io::Write;

use super::token::*;

pub mod display;
use crate::parser::display::*;
mod eval;
use crate::parser::eval::*;
mod test;


#[allow(clippy::cognitive_complexity)]
pub fn parse(mut tokens: &mut Tokens,
    mut registers: &mut [i32], mut hi: &mut u32, mut lo: &mut u32,
    mut data: &mut Vec<u8>, mut stack: &mut Vec<u8>)
-> Result<(), String> {

    data_analysis(&mut tokens, &mut data);
    //println!("data: {:?}", data);

    #[allow(unused)]
    while let Some(token) = tokens.consume() {
        //println!("{:?}", token); continue;

        // Skip until .text
        if TokenKind::INDICATE(IndicateKind::data) == token.kind {
            while let Some(token) = tokens.consume() {
                if TokenKind::INDICATE(IndicateKind::text) == token.kind {
                    break;
                }
            }
            if tokens.is_none() {
                break;
            }
        }

        // Skip LABEL, INDICATE and EOL
        if let TokenKind::LABEL(_, _, _) = tokens.kind() {
            tokens.consume().unwrap();
        }
        if let TokenKind::INDICATE(_) = tokens.kind() {
            while let TokenKind::INDICATE(_) = tokens.consume().unwrap().kind { continue; }
        }
        if tokens.expect_eol().is_ok() { continue; }

        let instruction_kind = tokens.expect_instruction()?;

        match instruction_kind {
            // Arithmetic, Logic
            InstructionKind::ADD |
            InstructionKind::ADDI =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| x + y),
            InstructionKind::ADDU |
            InstructionKind::ADDIU =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| (x as u32 + y as u32) as i32),
            InstructionKind::SUB =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| x - y),
            InstructionKind::SUBU =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| (x as u32 - y as u32) as i32),
            InstructionKind::MUL =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| x * y),
            InstructionKind::REM =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| x % y),
            InstructionKind::REMU =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| (x as u32 % y as u32) as i32),

            InstructionKind::DIV =>
                eval_arithmetic_hilo(&mut registers, &mut tokens, &mut hi, &mut lo, InstructionKind::DIV),
            InstructionKind::DIVU =>
                eval_arithmetic_hilo(&mut registers, &mut tokens, &mut hi, &mut lo, InstructionKind::DIVU),
            InstructionKind::MULT =>
                eval_arithmetic_hilo(&mut registers, &mut tokens, &mut hi, &mut lo, InstructionKind::MULT),
            InstructionKind::MULTU =>
                eval_arithmetic_hilo(&mut registers, &mut tokens, &mut hi, &mut lo, InstructionKind::MULTU),
            InstructionKind::MADD =>
                eval_arithmetic_hilo(&mut registers, &mut tokens, &mut hi, &mut lo, InstructionKind::MADD),
            InstructionKind::MADDU =>
                eval_arithmetic_hilo(&mut registers, &mut tokens, &mut hi, &mut lo, InstructionKind::MADDU),
            InstructionKind::MSUB =>
                eval_arithmetic_hilo(&mut registers, &mut tokens, &mut hi, &mut lo, InstructionKind::MSUB),
            InstructionKind::MSUBU =>
                eval_arithmetic_hilo(&mut registers, &mut tokens, &mut hi, &mut lo, InstructionKind::MSUBU),

            InstructionKind::MULO =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| x * y),
            InstructionKind::MULOU =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| (x as u32 * y as u32) as i32),
            InstructionKind::CLO =>
                eval_arithmetic(&mut registers, &mut tokens, move |x, _| {
                    let mut cnt: i32 = 0;
                    for i in (0..=31).rev() {
                        if (x as usize) >> i & 1 != 1 { break; }
                        cnt += 1;
                    }
                    cnt
                }),
            InstructionKind::CLZ =>
                eval_arithmetic(&mut registers, &mut tokens, move |x, _| {
                    let mut cnt: i32 = 0;
                    for i in (0..=31).rev() {
                        if (x as usize) >> i & 1 != 0 { break; }
                        cnt += 1;
                    }
                    cnt
                }),
            InstructionKind::ROR => {
                tokens.consume().unwrap();
                let rd_idx = tokens.expect_register()?;
                registers[rd_idx] = {
                    tokens.consume().unwrap();
                    let rs_idx = tokens.expect_register()?;
                    let rs = registers[rs_idx];
                    tokens.consume().unwrap();
                    let rt = {
                        if let Ok(rt_idx) = tokens.expect_register() {
                            registers[rt_idx]
                        } else if let Ok(num) = tokens.expect_integer() {
                            num
                        } else {
                            return Err("ROR: invalid token".into());
                        }
                    };
                    registers[1] = (rs as u32 >> rt) as i32;
                    registers[rd_idx] = (rs << (32-rt)) as i32;
                    registers[rd_idx] | registers[1]
                };
            },
            InstructionKind::ROL => {
                tokens.consume().unwrap();
                let rd_idx = tokens.expect_register()?;
                registers[rd_idx] = {
                    tokens.consume().unwrap();
                    let rs_idx = tokens.expect_register()?;
                    let rs = registers[rs_idx];
                    tokens.consume().unwrap();
                    let rt = {
                        if let Ok(rt_idx) = tokens.expect_register() {
                            registers[rt_idx]
                        } else if let Ok(num) = tokens.expect_integer() {
                            num
                        } else {
                            return Err("ROL: invalid token".into());
                        }
                    };
                    registers[1] = rs << rt;
                    registers[rd_idx] = (rs as u32 >> (32-rt)) as i32;
                    registers[rd_idx] | registers[1]
                };
            },

            InstructionKind::NOR =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| !(x | y)),
            InstructionKind::NOT => {
                tokens.consume().unwrap();
                let rd_idx = tokens.expect_register()?;
                registers[rd_idx] = {
                    tokens.consume().unwrap();
                    let register_idx = tokens.expect_register()?;
                    !registers[register_idx]
                };
            },
            InstructionKind::NEG |
            InstructionKind::NEGU => // UNSTABLE
                eval_arithmetic(&mut registers, &mut tokens, |x, _| -x),

            InstructionKind::SLL |
            InstructionKind::SLLV =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| x << y),
            InstructionKind::SRA |
            InstructionKind::SRAV =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| x >> y),
            InstructionKind::SRL |
            InstructionKind::SRLV =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| (x as u32 >> y) as i32),

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
                eval_constant(&mut registers, &mut tokens, |x| x & (std::u32::MAX-65535) as i32),

            // Comparison
            InstructionKind::SLT |
            InstructionKind::SLTI =>
                eval_comparison(&mut registers, &mut tokens, |x, y| x < y),
            InstructionKind::SEQ =>
                eval_comparison(&mut registers, &mut tokens, |x, y| x == y),
            InstructionKind::SGE =>
                eval_comparison(&mut registers, &mut tokens, |x, y| x >= y),
            InstructionKind::SGT =>
                eval_comparison(&mut registers, &mut tokens, |x, y| x > y),
            InstructionKind::SLE =>
                eval_comparison(&mut registers, &mut tokens, |x, y| x <= y),
            InstructionKind::SNE =>
                eval_comparison(&mut registers, &mut tokens, |x, y| x != y),

            // Branch
            InstructionKind::B =>
                if eval_branch(&mut registers, &mut tokens, |_, _| true)   { continue; },
            InstructionKind::BEQ =>
                if eval_branch(&mut registers, &mut tokens, |x, y| x == y) { continue; },
            InstructionKind::BNE =>
                if eval_branch(&mut registers, &mut tokens, |x, y| x != y) { continue; },
            InstructionKind::BGE =>
                if eval_branch(&mut registers, &mut tokens, |x, y| x >= y) { continue; },
            InstructionKind::BGT =>
                if eval_branch(&mut registers, &mut tokens, |x, y| x > y)  { continue; },
            InstructionKind::BLE =>
                if eval_branch(&mut registers, &mut tokens, |x, y| x <= y) { continue; },
            InstructionKind::BLT =>
                if eval_branch(&mut registers, &mut tokens, |x, y| x < y)  { continue; },
            InstructionKind::BEQZ =>
                if eval_branch(&mut registers, &mut tokens, |x, y| x == y) { continue; },
            InstructionKind::BGEZ =>
                if eval_branch(&mut registers, &mut tokens, |x, y| x >= y) { continue; },
            InstructionKind::BGTZ =>
                if eval_branch(&mut registers, &mut tokens, |x, y| x > y)  { continue; },
            InstructionKind::BLEZ =>
                if eval_branch(&mut registers, &mut tokens, |x, y| x <= y) { continue; },
            InstructionKind::BLTZ =>
                if eval_branch(&mut registers, &mut tokens, |x, y| x < y)  { continue; },
            InstructionKind::BNEZ =>
                if eval_branch(&mut registers, &mut tokens, |x, y| x != y) { continue; },

            // Jump
            InstructionKind::J =>
                if eval_jump(&mut registers, &mut tokens, InstructionKind::J)    { continue; },
            InstructionKind::JAL =>
                if eval_jump(&mut registers, &mut tokens, InstructionKind::JAL)  { continue; },
            InstructionKind::JR =>
                if eval_jump(&mut registers, &mut tokens, InstructionKind::JR)   { continue; },
            InstructionKind::JALR =>
                if eval_jump(&mut registers, &mut tokens, InstructionKind::JALR) { continue; },

            // Load
            InstructionKind::LA => {
                tokens.consume().unwrap();
                let register_idx = tokens.expect_register()?;
                tokens.consume().unwrap();
                let label_idx = tokens.expect_address()? as i32;
                registers[register_idx] = label_idx;
            },
            InstructionKind::LB =>  // Rt = *((int*)address) (8bit)
                eval_load(&mut registers, &mut tokens, &data, &mut stack, 1),
            InstructionKind::LH =>  // Rt = *((int*)address) (16bit)
                eval_load(&mut registers, &mut tokens, &data, &mut stack, 2),
            InstructionKind::LW =>  // Rt = *((int*)address) (32bit)
                eval_load(&mut registers, &mut tokens, &data, &mut stack, 4),

            // Store
            InstructionKind::SW => {  // *((int*)address) = Rt (32bit)
                tokens.consume().unwrap();
                let register_idx = tokens.expect_register()?;
                tokens.consume().unwrap();
                if let Ok((r_idx, s_idx)) = tokens.expect_stack() {
                    let stack_idx = -(registers[r_idx] + s_idx) as usize;
                    if stack.len() <= stack_idx {
                        stack.resize(stack_idx+1, 0);
                    }
                    stack[stack_idx-3] = (registers[register_idx]>>24) as u8;
                    stack[stack_idx-2] = (registers[register_idx]>>16) as u8;
                    stack[stack_idx-1] = (registers[register_idx]>> 8) as u8;
                    stack[stack_idx]   = (registers[register_idx]    ) as u8;
                } else {
                    let (r_idx, d_idx) = tokens.expect_data()?;
                    let index = registers[r_idx] as usize + d_idx - 1;
                    data[index]   = (registers[register_idx]>>24) as u8;
                    data[index+1] = (registers[register_idx]>>16) as u8;
                    data[index+2] = (registers[register_idx]>> 8) as u8;
                    data[index+3] = (registers[register_idx]    ) as u8;
                }
            },

            // Transfer
            InstructionKind::MOVE => {
                tokens.consume().unwrap();
                let register_idx = tokens.expect_register()?;
                registers[register_idx] = {
                    let r1_idx = if tokens.consume().is_some() {
                        tokens.expect_register()?
                    } else {
                        // TODO
                        //todo!();
                        panic!("TODO");
                    };
                    registers[r1_idx]
                };
            },

            // Exception, Interrupt
            InstructionKind::SYSCALL => {
                match registers[2] {  // v0
                    // print_int: $a0=integer
                    1  => {
                        print!("{}", registers[4]);  // $a0
                        std::io::stdout().flush().unwrap();
                    },
                    // print_string: $a0=string(data index)
                    4  => {
                        print!("{}", get_string(&data, &stack, registers[4]));  // $a0
                        std::io::stdout().flush().unwrap();
                    },
                    // read_int: return $v0
                    5  => {
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();
                        registers[2] = input.trim().parse::<i32>().unwrap();  // $v0
                    },
                    // read_string: $a0=buffer, $a1=length.  write buffer
                    8  => {
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();
                        let mut index = registers[4] as usize - 1;
                        if index >= data.len() {
                            panic!("invalid address for .space: {}", registers[4]);
                        }
                        for (i, ch) in input.into_bytes().iter().enumerate() {
                            if i >= registers[5] as usize {
                                break;
                            }
                            data[index] = *ch;
                            index += 1;
                        }
                    },
                    // exit
                    10 => {
                        for r in registers { *r = 0; }
                        *hi = 0;
                        *lo = 0;
                        data.clear();
                        stack.clear();
                        tokens.init();
                        break;
                    },
                    _ => return Err(format!("SYSCALL: invalid code: {}", registers[2])),
                }
            },
            InstructionKind::NOP => (),  // Do nothing

            // My own
            InstructionKind::PRTN =>
                eval_myown(&registers, &mut tokens, &data, &stack, InstructionKind::PRTN),
            InstructionKind::PRTI =>
                eval_myown(&registers, &mut tokens, &data, &stack, InstructionKind::PRTI),
            InstructionKind::PRTH =>
                eval_myown(&registers, &mut tokens, &data, &stack, InstructionKind::PRTH),
            InstructionKind::PRTX =>
                eval_myown(&registers, &mut tokens, &data, &stack, InstructionKind::PRTX),
            InstructionKind::PRTC =>
                eval_myown(&registers, &mut tokens, &data, &stack, InstructionKind::PRTC),
            InstructionKind::PRTS =>
                eval_myown(&registers, &mut tokens, &data, &stack, InstructionKind::PRTS),
            InstructionKind::RST => {
                for r in registers { *r = 0; }
                *hi = 0;
                *lo = 0;
                data.clear();
                stack.clear();
                tokens.init();
                break;
            },
            //_ => (),
        }

        // expect TokenKind::EOL
        tokens.consume();
        tokens.expect_eol()?;

        if std::env::var("DATA_TRACE").is_ok() {
            display_data_per_4byte(&data);
        }
        if std::env::var("STACK_TRACE").is_ok() {
            display_stack(&stack);
        }
        if std::env::var("REGISTER_TRACE").is_ok() {
            display_register(&registers);
        }
    }

    //display_data_per_4byte(&data);
    //display_stack(&stack);
    //display_register(&registers);
    Ok(())
}

/// Return signed integer (32-bit)
///
/// # Example
///
/// ```rust
/// let int: i32 = get_int(&data, &stack, registers[4]);
/// ```
///
/// argument1: data:&[u8]
/// argument2: stack:&[u8]
/// argument3: index: isize  =>  stack(-) | data(+)
/// argument4: byte
pub fn get_int(data: &[u8], stack: &[u8], index: isize, byte: usize) -> i32 {

    // stack
    if index < 0 {
        let index = (-index - 1) as usize;
        let mut int: i32 = 0;
        // Big Endian
        for i in 0..byte {
            int |= (stack[index+i] as i32) << ((byte-1-i) * 8);
        }

        int

    // data
    } else if 0 < index {
        let index = ( index - 1) as usize;
        let mut int: i32 = 0;
        // Big Endian
        for i in 0..byte {
            int |= (data[index+i] as i32) << ((byte-1-i) * 8);
        }

        int

    } else {
        panic!(format!("get_int(): invalid index: {}", index));
    }
}

pub fn get_string(data: &[u8], stack: &[u8], index: i32) -> String {
    // stack
    if index < 0 {
        let mut i = (-index - 1) as usize;
        let mut s = String::new();
        let stack_len = stack.len();

        while i < stack_len && stack[i] != 0 {
            s = format!("{}{}", s, stack[i] as char);
            i += 1;
        }
        s

    // data
    } else if 0 < index {
        let mut i = (index - 1) as usize;
        let mut s = String::new();
        let data_len = data.len();

        while i < data_len && data[i] != 0 {
            s = format!("{}{}", s, data[i] as char);
            i += 1;
        }
        s

    } else {
        panic!(format!("get_string(): invalid index: {}", index));
    }
}

/// Push to data: &Vec<u8> from .data segment's data
fn data_analysis(tokens: &mut Tokens, data: &mut Vec<u8>) {

    let old_idx = tokens.idx();
    tokens.goto(if old_idx == 0 {0} else {old_idx-1});

    // Check all tokens
    while tokens.consume().is_some() {

        // Ignore except .data|.align segment
        if tokens.data_area_now || match tokens.token[tokens.idx()].kind {
            TokenKind::INDICATE(IndicateKind::data) =>
                true,
            TokenKind::INDICATE(IndicateKind::align(n)) => {
                let padding = 2i32.pow(n as u32) as usize;
                let i = data.len() % padding;
                for _ in 0..i {
                    data.push(0);
                }
                false
            },
            _ => false,
        } {

            tokens.data_area_now = true;

            // consume EOL
            while let Some(token) = tokens.next() {
                if token.kind == TokenKind::EOL {
                    tokens.consume().unwrap();
                } else {
                    break;
                }
            }

            // until .text segment
            while tokens.consume().is_some() {
                if TokenKind::INDICATE(IndicateKind::text) == *tokens.kind() {
                    tokens.data_area_now = false;
                    break;
                }

                // TokenKind::LABEL(usize) = data.len() + 1
                if let TokenKind::LABEL(_, _, ref mut index) = &mut tokens.kind() {
                    *index = Some(data.len() + 1);
                    if tokens.next().unwrap().kind == TokenKind::EOL {
                        tokens.consume().unwrap();
                    }
                }

                match &*tokens.kind() {
                    // Align 2^n
                    TokenKind::INDICATE(IndicateKind::align(n)) => {
                        let padding = 2i32.pow(*n as u32) as usize;
                        let i = data.len() % padding;
                        for _ in 0..i {
                            data.push(0);
                        }
                    },
                    TokenKind::INDICATE(IndicateKind::word(w)) => {
                        data.push((*w>>24) as u8);
                        data.push((*w>>16) as u8);
                        data.push((*w>> 8) as u8);
                        data.push( *w      as u8);
                    },
                    TokenKind::INDICATE(IndicateKind::half(h)) => {
                        data.push((*h>> 8) as u8);
                        data.push( *h      as u8);
                    },
                    TokenKind::INDICATE(IndicateKind::byte(b)) => {
                        data.push(*b);
                    },
                    _ => (),
                }

                // until EOL
                while let Some(token) = tokens.consume() {
                    if token.kind == TokenKind::EOL {
                        break;
                    }

                    match tokens.kind() {
                        // Big Endian
                        TokenKind::INDICATE(IndicateKind::word(w)) => {
                            data.push((*w>>24) as u8);
                            data.push((*w>>16) as u8);
                            data.push((*w>> 8) as u8);
                            data.push( *w      as u8);
                        },
                        TokenKind::INDICATE(IndicateKind::half(h)) => {
                            data.push((*h>> 8) as u8);
                            data.push( *h      as u8);
                        },
                        TokenKind::INDICATE(IndicateKind::byte(b)) => {
                            data.push(*b);
                        },
                        TokenKind::INDICATE(IndicateKind::space(s)) => {
                            for _ in 0..*s {
                                data.push(0);
                            }
                        },
                        TokenKind::INDICATE(IndicateKind::ascii(s)) => {
                            for ch in s.bytes() {
                                data.push(ch);
                            }
                        },
                        TokenKind::INDICATE(IndicateKind::asciiz(s)) => {
                            for ch in s.bytes() {
                                data.push(ch);
                            }
                            data.push(0);
                        },
                        _ => (),
                    }
                }
            }
        }
    }

    tokens.goto(old_idx);
}

