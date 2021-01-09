extern crate rand;
use rand::Rng;

use std::io::Write;
use std::error::Error;

use super::token::*;
use super::token::register::{Registers, RegisterKind::*};
use super::token::memory::*;

pub mod display;
use crate::parser::display::*;
mod eval;
use crate::parser::eval::*;
mod test;


#[allow(clippy::cognitive_complexity)]
pub fn parse(mut tokens: &mut Tokens, mut memory: &mut Memory) -> Result<(), Box<dyn Error>> {

    data_analysis(&mut tokens, &mut memory.static_data);
    //println!("data: {:?}", data);
    //println!("tokens: {:?}", tokens);

    while tokens.consume().is_some() {
        //println!("{:?}", *tokens.kind()); continue;

        // Skip until .text
        if TokenKind::INDICATE(IndicateKind::data) == *tokens.kind() {
            while tokens.consume().is_some() {
                if TokenKind::INDICATE(IndicateKind::text) == *tokens.kind() {
                    break;
                }
            }
            if tokens.is_none() {
                break;
            }
        }

        // Skip LABEL, INDICATE and EOL
        while matches!(*tokens.kind(), TokenKind::LABEL(_, _, _) | TokenKind::INDICATE(_) | TokenKind::EOL) {
            if tokens.consume().is_none() {
                return Ok(());
            }
        }

        let instruction_kind = tokens.expect_instruction()?;

        match instruction_kind {
            // Arithmetic, Logic
            InstructionKind::ADD |
            InstructionKind::ADDI =>
                eval_arithmetic(&mut memory.registers, &mut tokens, |x, y| x.checked_add(y))?,
            InstructionKind::ADDU |
            InstructionKind::ADDIU =>
                eval_arithmetic(&mut memory.registers, &mut tokens, |x, y| Some(x + y))?,
            InstructionKind::SUB =>
                eval_arithmetic(&mut memory.registers, &mut tokens, |x, y| x.checked_sub(y))?,
            InstructionKind::SUBU =>
                eval_arithmetic(&mut memory.registers, &mut tokens, |x, y| Some(x - y))?,
            InstructionKind::MUL => {
                //eval_arithmetic(&mut memory.registers, &mut tokens, |x, y| x.checked_mul(y))?, // TODO: mult $2,$3;mflo $1
                tokens.consume().ok_or(CONSUME_ERR)?;
                let r1_idx = tokens.expect_register()?;
                tokens.consume().ok_or(CONSUME_ERR)?;
                let r2_idx = tokens.expect_register()?;
                if let TokenKind::REGISTER(_, r3_idx) = tokens.next().unwrap().kind {
                    tokens.consume().ok_or(CONSUME_ERR)?;
                    // mult $2, $3
                    let ans = memory.registers[r2_idx] as i64 * memory.registers[r3_idx] as i64;
                    memory.lo = ans as u32;
                    memory.hi = ((ans as u64) >> 32) as u32;
                    // mflo $1
                    memory.registers[r1_idx] = memory.lo as i32;
                } else {
                    // mult
                    let ans = memory.registers[r1_idx] as i64 * memory.registers[r2_idx] as i64;
                    memory.lo = ans as u32;
                    memory.hi = ((ans as u64) >> 32) as u32;
                }
            },
            InstructionKind::REM =>
                eval_arithmetic(&mut memory.registers, &mut tokens, |x, y| Some(x % y))?,
            InstructionKind::REMU =>
                eval_arithmetic(&mut memory.registers, &mut tokens, |x, y| x.checked_rem(y))?,

            InstructionKind::DIV =>
                eval_arithmetic_hilo(&mut memory, &mut tokens, InstructionKind::DIV)?,
            InstructionKind::DIVU =>
                eval_arithmetic_hilo(&mut memory, &mut tokens, InstructionKind::DIVU)?,
            InstructionKind::MULT =>
                eval_arithmetic_hilo(&mut memory, &mut tokens, InstructionKind::MULT)?,
            InstructionKind::MULTU =>
                eval_arithmetic_hilo(&mut memory, &mut tokens, InstructionKind::MULTU)?,
            InstructionKind::MADD =>
                eval_arithmetic_hilo(&mut memory, &mut tokens, InstructionKind::MADD)?,
            InstructionKind::MADDU =>
                eval_arithmetic_hilo(&mut memory, &mut tokens, InstructionKind::MADDU)?,
            InstructionKind::MSUB =>
                eval_arithmetic_hilo(&mut memory, &mut tokens, InstructionKind::MSUB)?,
            InstructionKind::MSUBU =>
                eval_arithmetic_hilo(&mut memory, &mut tokens, InstructionKind::MSUBU)?,

            InstructionKind::MULO =>
                eval_arithmetic(&mut memory.registers, &mut tokens, |x, y| Some(x * y))?,
            InstructionKind::MULOU =>
                eval_arithmetic(&mut memory.registers, &mut tokens, |x, y| Some((x as u32 * y as u32) as i32))?,
            InstructionKind::CLO =>
                eval_arithmetic(&mut memory.registers, &mut tokens, move |x, _| {
                    let mut cnt: i32 = 0;
                    for i in (0..=31).rev() {
                        if (x as usize) >> i & 1 != 1 { break; }
                        cnt += 1;
                    }
                    Some(cnt)
                })?,
            InstructionKind::CLZ =>
                eval_arithmetic(&mut memory.registers, &mut tokens, move |x, _| {
                    let mut cnt: i32 = 0;
                    for i in (0..=31).rev() {
                        if (x as usize) >> i & 1 != 0 { break; }
                        cnt += 1;
                    }
                    Some(cnt)
                })?,
            InstructionKind::ROR => {
                tokens.consume().ok_or(CONSUME_ERR)?;
                let rd_idx = tokens.expect_register()?;
                memory.registers[rd_idx] = {
                    tokens.consume().ok_or(CONSUME_ERR)?;
                    let rs_idx = tokens.expect_register()?;
                    let rs = memory.registers[rs_idx];
                    tokens.consume().ok_or(CONSUME_ERR)?;
                    let rt = {
                        if let Ok(rt_idx) = tokens.expect_register() {
                            memory.registers[rt_idx]
                        } else if let Ok(num) = tokens.expect_integer() {
                            num
                        } else {
                            return Err("ROR: invalid token".into());
                        }
                    };
                    memory.registers[at] = (rs as u32 >> rt) as i32;
                    memory.registers[rd_idx] = (rs << (32-rt)) as i32;
                    memory.registers[rd_idx] | memory.registers[at]
                };
            },
            InstructionKind::ROL => {
                tokens.consume().ok_or(CONSUME_ERR)?;
                let rd_idx = tokens.expect_register()?;
                memory.registers[rd_idx] = {
                    tokens.consume().ok_or(CONSUME_ERR)?;
                    let rs_idx = tokens.expect_register()?;
                    let rs = memory.registers[rs_idx];
                    tokens.consume().ok_or(CONSUME_ERR)?;
                    let rt = {
                        if let Ok(rt_idx) = tokens.expect_register() {
                            memory.registers[rt_idx]
                        } else if let Ok(num) = tokens.expect_integer() {
                            num
                        } else {
                            return Err("ROL: invalid token".into());
                        }
                    };
                    memory.registers[at] = rs << rt;
                    memory.registers[rd_idx] = (rs as u32 >> (32-rt)) as i32;
                    memory.registers[rd_idx] | memory.registers[at]
                };
            },

            InstructionKind::NOR =>
                eval_arithmetic(&mut memory.registers, &mut tokens, |x, y| Some(!(x | y)))?,
            InstructionKind::NOT => {
                tokens.consume().ok_or(CONSUME_ERR)?;
                let rd_idx = tokens.expect_register()?;
                memory.registers[rd_idx] = {
                    tokens.consume().ok_or(CONSUME_ERR)?;
                    let register_idx = tokens.expect_register()?;
                    !memory.registers[register_idx]
                };
            },
            InstructionKind::NEG =>
                eval_arithmetic(&mut memory.registers, &mut tokens, |x, _| Some(-x))?, // TODO (with overflow)
            InstructionKind::NEGU =>
                eval_arithmetic(&mut memory.registers, &mut tokens, |x, _| Some(-x))?,

            InstructionKind::SLL |
            InstructionKind::SLLV =>
                eval_arithmetic(&mut memory.registers, &mut tokens, |x, y| Some(x << y))?,
            InstructionKind::SRA |
            InstructionKind::SRAV =>
                eval_arithmetic(&mut memory.registers, &mut tokens, |x, y| Some(x >> y))?,
            InstructionKind::SRL |
            InstructionKind::SRLV =>
                eval_arithmetic(&mut memory.registers, &mut tokens, |x, y| Some((x as u32 >> y) as i32))?,

            InstructionKind::AND |
            InstructionKind::ANDI =>
                eval_arithmetic(&mut memory.registers, &mut tokens, |x, y| Some(x & y))?,
            InstructionKind::OR |
            InstructionKind::ORI =>
                eval_arithmetic(&mut memory.registers, &mut tokens, |x, y| Some(x | y))?,
            InstructionKind::XOR |
            InstructionKind::XORI =>
                eval_arithmetic(&mut memory.registers, &mut tokens, |x, y| Some(x ^ y))?,

            // Constant
            InstructionKind::LI =>
                eval_constant(&mut memory.registers, &mut tokens, |x| x)?,
            InstructionKind::LUI =>
                eval_constant(&mut memory.registers, &mut tokens, |x| x & (std::u32::MAX-65535) as i32)?,

            // Comparison
            InstructionKind::SLT |
            InstructionKind::SLTI =>
                eval_comparison(&mut memory.registers, &mut tokens, |x, y| x < y)?,
            InstructionKind::SEQ =>
                eval_comparison(&mut memory.registers, &mut tokens, |x, y| x == y)?,
            InstructionKind::SGE =>
                eval_comparison(&mut memory.registers, &mut tokens, |x, y| x >= y)?,
            InstructionKind::SGT =>
                eval_comparison(&mut memory.registers, &mut tokens, |x, y| x > y)?,
            InstructionKind::SLE =>
                eval_comparison(&mut memory.registers, &mut tokens, |x, y| x <= y)?,
            InstructionKind::SNE =>
                eval_comparison(&mut memory.registers, &mut tokens, |x, y| x != y)?,

            // Branch
            InstructionKind::B =>
                if eval_branch(&mut memory.registers, &mut tokens, |_, _| true)?   { continue; },
            InstructionKind::BEQ =>
                if eval_branch(&mut memory.registers, &mut tokens, |x, y| x == y)? { continue; },
            InstructionKind::BNE =>
                if eval_branch(&mut memory.registers, &mut tokens, |x, y| x != y)? { continue; },
            InstructionKind::BGE =>
                if eval_branch(&mut memory.registers, &mut tokens, |x, y| x >= y)? { continue; },
            InstructionKind::BGT =>
                if eval_branch(&mut memory.registers, &mut tokens, |x, y| x > y)?  { continue; },
            InstructionKind::BLE =>
                if eval_branch(&mut memory.registers, &mut tokens, |x, y| x <= y)? { continue; },
            InstructionKind::BLT =>
                if eval_branch(&mut memory.registers, &mut tokens, |x, y| x < y)?  { continue; },
            InstructionKind::BEQZ =>
                if eval_branch(&mut memory.registers, &mut tokens, |x, y| x == y)? { continue; },
            InstructionKind::BGEZ =>
                if eval_branch(&mut memory.registers, &mut tokens, |x, y| x >= y)? { continue; },
            InstructionKind::BGTZ =>
                if eval_branch(&mut memory.registers, &mut tokens, |x, y| x > y)?  { continue; },
            InstructionKind::BLEZ =>
                if eval_branch(&mut memory.registers, &mut tokens, |x, y| x <= y)? { continue; },
            InstructionKind::BLTZ =>
                if eval_branch(&mut memory.registers, &mut tokens, |x, y| x < y)?  { continue; },
            InstructionKind::BNEZ =>
                if eval_branch(&mut memory.registers, &mut tokens, |x, y| x != y)? { continue; },
            InstructionKind::BGEZAL => {
                tokens.consume().ok_or(CONSUME_ERR)?;
                let r_idx = tokens.expect_register()?;
                tokens.consume().ok_or(CONSUME_ERR)?;
                let l_idx = tokens.expect_label()?;
                memory.registers[ra] = tokens.idx() as i32 + 1;
                if 0 <= memory.registers[r_idx] {
                    tokens.goto(l_idx-1);
                }
            },
            InstructionKind::BLTZAL => {
                tokens.consume().ok_or(CONSUME_ERR)?;
                let r_idx = tokens.expect_register()?;
                tokens.consume().ok_or(CONSUME_ERR)?;
                let l_idx = tokens.expect_label()?;
                memory.registers[ra] = tokens.idx() as i32 + 1;
                if memory.registers[r_idx] < 0 {
                    tokens.goto(l_idx-1);
                }
            },

            // Jump
            InstructionKind::J =>
                { eval_jump(&mut memory.registers, &mut tokens, InstructionKind::J)?;    continue; },
            InstructionKind::JAL =>
                { eval_jump(&mut memory.registers, &mut tokens, InstructionKind::JAL)?;  continue; },
            InstructionKind::JR =>
                { eval_jump(&mut memory.registers, &mut tokens, InstructionKind::JR)?;   continue; },
            InstructionKind::JALR =>
                { eval_jump(&mut memory.registers, &mut tokens, InstructionKind::JALR)?; continue; },

            // Load
            InstructionKind::LA => {
                tokens.consume().ok_or(CONSUME_ERR)?;
                let register_idx = tokens.expect_register()?;
                tokens.consume().ok_or(CONSUME_ERR)?;
                memory.registers[register_idx] = {
                    if let Ok(data_idx) = tokens.expect_address() {
                        data_idx as i32
                    } else {
                        tokens.expect_label().unwrap() as i32
                    }
                };
            },
            InstructionKind::LB =>   // Rt = *((int*)address) (8bit)
                eval_load(&mut memory, &mut tokens, 1, SignExtension::Signed)?,
            InstructionKind::LBU =>  // Rt = *((int*)address) (8bit)
                eval_load(&mut memory, &mut tokens, 1, SignExtension::Unsigned)?,
            InstructionKind::LH =>   // Rt = *((int*)address) (16bit)
                eval_load(&mut memory, &mut tokens, 2, SignExtension::Signed)?,
            InstructionKind::LHU =>  // Rt = *((int*)address) (16bit)
                eval_load(&mut memory, &mut tokens, 2, SignExtension::Unsigned)?,
            InstructionKind::LW =>   // Rt = *((int*)address) (32bit)
                eval_load(&mut memory, &mut tokens, 4, SignExtension::Unsigned)?,

            // Store
            InstructionKind::SB =>  // *((int*)address) = Rt (8bit)
                eval_store(&mut memory, &mut tokens, 1)?,
            InstructionKind::SH =>  // *((int*)address) = Rt (16bit)
                eval_store(&mut memory, &mut tokens, 2)?,
            InstructionKind::SW =>  // *((int*)address) = Rt (32bit)
                eval_store(&mut memory, &mut tokens, 4)?,

            // Transfer
            InstructionKind::MOVE => {
                tokens.consume().ok_or(CONSUME_ERR)?;
                let rd_idx = tokens.expect_register()?;
                tokens.consume().ok_or(CONSUME_ERR)?;
                let rs_idx = tokens.expect_register()?;
                memory.registers[rd_idx] = memory.registers[rs_idx];
            },
            InstructionKind::MFHI => {
                tokens.consume().ok_or(CONSUME_ERR)?;
                let r_idx = tokens.expect_register()?;
                memory.registers[r_idx] = memory.hi as i32;
            },
            InstructionKind::MFLO => {
                tokens.consume().ok_or(CONSUME_ERR)?;
                let r_idx = tokens.expect_register()?;
                memory.registers[r_idx] = memory.lo as i32;
            },
            InstructionKind::MTHI => {
                tokens.consume().ok_or(CONSUME_ERR)?;
                let r_idx = tokens.expect_register()?;
                memory.hi = memory.registers[r_idx] as u32;
            },
            InstructionKind::MTLO => {
                tokens.consume().ok_or(CONSUME_ERR)?;
                let r_idx = tokens.expect_register()?;
                memory.lo = memory.registers[r_idx] as u32;
            },
            InstructionKind::MOVN => {
                tokens.consume().ok_or(CONSUME_ERR)?;
                let rd_idx = tokens.expect_register()?;
                tokens.consume().ok_or(CONSUME_ERR)?;
                let rs_idx = tokens.expect_register()?;
                tokens.consume().ok_or(CONSUME_ERR)?;
                let rt_idx = tokens.expect_register()?;
                if memory.registers[rt_idx] != 0 {
                    memory.registers[rd_idx] = memory.registers[rs_idx];
                }
            },
            InstructionKind::MOVZ => {
                tokens.consume().ok_or(CONSUME_ERR)?;
                let rd_idx = tokens.expect_register()?;
                tokens.consume().ok_or(CONSUME_ERR)?;
                let rs_idx = tokens.expect_register()?;
                tokens.consume().ok_or(CONSUME_ERR)?;
                let rt_idx = tokens.expect_register()?;
                if memory.registers[rt_idx] == 0 {
                    memory.registers[rd_idx] = memory.registers[rs_idx];
                }
            },

            // Exception, Interrupt
            InstructionKind::SYSCALL => {
                match memory.registers[v0] {
                    // print_int: $a0=integer
                    1  => {
                        print!("{}", memory.registers[a0]);
                        let _ = std::io::stdout().flush();
                    },
                    // print_float: $f12=integer
                    2  => {
                        print!("{}", f32::from_bits(memory.registers[f12] as u32));
                        let _ = std::io::stdout().flush();
                    },
                    // print_string: $a0=string(data index)
                    4  => {
                        print!("{}", get_string(&memory, memory.registers[a0] as u32)?);
                        let _ = std::io::stdout().flush();
                    },
                    // read_int: return $v0
                    5  => {
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();
                        memory.registers[v0] = if let Ok(num) = input.trim().parse::<i32>() {
                            num
                        } else {
                            0
                        };
                    },
                    // read_string: $a0=buffer, $a1=length.  write buffer
                    8  => {
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();
                        let mut index = memory.registers[a0] as usize - 1;
                        if memory.static_data.len() < index + input.len() {  // TODO
                            return Err(format!("not enough space for .data: {}", memory.registers[a0]).into());
                        }
                        for (i, ch) in input.into_bytes().iter().enumerate() {
                            if i >= memory.registers[a1] as usize {
                                break;
                            }
                            memory.static_data[index] = *ch;
                            index += 1;
                        }
                    },
                    // sbrk(allocate heap memory): $a0=size. $v0=address
                    9 => {
                        let size = memory.registers[a0];
                        memory.registers[v0] = memory.malloc(size)?;
                    },
                    // exit
                    10 => {
                        reset(&mut memory, &mut tokens);
                        break;
                    },
                    // print_character
                    11 => {
                        print!("{}", memory.registers[a0] as u8 as char);
                        let _ = std::io::stdout().flush();
                    },
                    // read character
                    12 => {
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();
                        memory.registers[v0] = input.as_bytes()[0] as i32;
                    },
                    // exit2
                    17 => {
                        std::process::exit(memory.registers[a0]);
                    },
                    // random_int:
                    // $a0 = random number(int)
                    41 => {
                        let rnd = rand::thread_rng().gen();
                        memory.registers[a0] = rnd;
                    },
                    // random_int_range:
                    // $a0 = random number(int)
                    // $a1 = upper bound of range of returned valus.
                    42 => {
                        let rnd = rand::thread_rng().gen_range(0, memory.registers[a1]);
                        memory.registers[a0] = rnd;
                    },
                    _ => return Err(format!("SYSCALL: invalid code: {}", memory.registers[v0]).into()),
                }
            },
            InstructionKind::NOP => (),  // Do nothing

            // My own
            InstructionKind::PRTN =>
                eval_myown(&memory, &mut tokens, InstructionKind::PRTN)?,
            InstructionKind::PRTI =>
                eval_myown(&memory, &mut tokens, InstructionKind::PRTI)?,
            InstructionKind::PRTH =>
                eval_myown(&memory, &mut tokens, InstructionKind::PRTH)?,
            InstructionKind::PRTX =>
                eval_myown(&memory, &mut tokens, InstructionKind::PRTX)?,
            InstructionKind::PRTC =>
                eval_myown(&memory, &mut tokens, InstructionKind::PRTC)?,
            InstructionKind::PRTS =>
                eval_myown(&memory, &mut tokens, InstructionKind::PRTS)?,
            InstructionKind::RST => {
                reset(&mut memory, &mut tokens);
                break;
            },

            // FPU Instructions
            InstructionKind::ADD_S =>
                eval_fp_arithmetic(&mut memory.registers, &mut tokens, InstructionKind::ADD_S)?,
            InstructionKind::SUB_S =>
                eval_fp_arithmetic(&mut memory.registers, &mut tokens, InstructionKind::SUB_S)?,
            InstructionKind::DIV_S =>
                eval_fp_arithmetic(&mut memory.registers, &mut tokens, InstructionKind::DIV_S)?,
            InstructionKind::MUL_S =>
                eval_fp_arithmetic(&mut memory.registers, &mut tokens, InstructionKind::MUL_S)?,
            InstructionKind::MTC1 => {
                tokens.consume().ok_or(CONSUME_ERR)?;
                let rs_idx = tokens.expect_register()?;
                tokens.consume().ok_or(CONSUME_ERR)?;
                let rd_idx = tokens.expect_register()?;
                memory.registers[rd_idx] = (memory.registers[rs_idx] as f32).to_bits() as i32;
            },
            InstructionKind::CVT_S_W => {
                tokens.consume().ok_or(CONSUME_ERR)?;
                let rd_idx = tokens.expect_register()?;
                memory.registers[rd_idx] = {
                    tokens.consume().ok_or(CONSUME_ERR)?;
                    let rs_idx = tokens.expect_register()?;
                    (memory.registers[rs_idx] as f32).to_bits() as i32
                };
            },

            //_ => (),
        }

        // expect TokenKind::EOL
        tokens.consume();
        tokens.expect_eol()?;

        if tokens.data_trace() {
            display_data_per_4byte(&memory.static_data);
        }
        if tokens.stack_trace() {
            display_stack(&memory.stack);
        }
        if tokens.register_trace() {
            display_register(&memory.registers);
        }
        if tokens.fp_register_trace() {
            display_fp_register(&memory.registers);
        }
    }

    //display_data_per_4byte(&data);
    //display_stack(&stack);
    //display_register(&registers);
    Ok(())
}

pub enum SignExtension {
    Signed,
    Unsigned,
}

/// Return signed integer (32-bit)
///
/// # Example
///
/// ```rust,ignore
/// let int: i32 = get_int(&memory, registers[a0], 4, SignExtension::Signed)?;
/// ```
///
/// argument1: data:&[u8]
/// argument2: stack:&[u8]
/// argument3: index: isize  =>  stack(<=0) | data(0<)
/// argument4: byte
pub fn get_int(memory: &Memory, index: u32, byte: usize, se: SignExtension) -> Result<i32, String> {
    let mut int: u32 = 0;

    // data
    if index < DYNAMIC_DATA_EXIT {
        if index < DYNAMIC_DATA {
            // static data
            let index = (index - if STATIC_DATA <= index { STATIC_DATA } else { 0 } - 1) as usize;
            if memory.static_data.len() < index+byte {
                return Err(
                    format!("get_int(): index out of bounds: the static-data len is {}, but the index is {}-{}",
                    memory.static_data.len(), index, index+byte-1));
            }
            // Big Endian
            for i in 0..byte {
                int |= (memory.static_data[index+i] as u32) << ((byte-1-i) * 8);
            }
        } else {
            // dynamic data
            let index = (index - DYNAMIC_DATA) as usize;
            if memory.dynamic_data.len() < index+byte {
                return Err(
                    format!("get_int(): index out of bounds: the dynamic-data len is {}, but the index is {}-{}",
                    memory.dynamic_data.len(), index, index+byte-1));
            }
            // Big Endian
            for i in 0..byte {
                int |= (memory.dynamic_data[index+i] as u32) << ((byte-1-i) * 8);
            }
        }


    // stack
    } else {
        let index = (STACK_SEGMENT - index) as usize;
        if memory.stack.len() < index+byte {
            return Err(
                format!("get_int(): index out of bounds: the stack len is {}, but the index is {}-{}",
                memory.stack.len(), index, index+byte-1));
        }
        // Big Endian
        for i in 0..byte {
            int |= (memory.stack[index+i] as u32) << ((byte-1-i) * 8);
        }
    }

    match se {
        SignExtension::Signed   => Ok(-(int as i32)),
        SignExtension::Unsigned => Ok(  int as i32),
    }
}

pub fn get_string(memory: &Memory, index: u32) -> Result<String, String> {
    // data
    if index < DYNAMIC_DATA_EXIT {
        if index < DYNAMIC_DATA {
            // static data
            let mut i = (index - if STATIC_DATA <= index { STATIC_DATA } else { 0 } - 1) as usize;
            let mut s = String::new();
            let data_len = memory.static_data.len();

            while i < data_len && memory.static_data[i] != 0 {
                s.push(memory.static_data[i] as char);
                i += 1;
            }

            Ok(s)
        } else {
            // dynamic data
            let mut i = (index - DYNAMIC_DATA) as usize;
            let mut s = String::new();
            let data_len = memory.dynamic_data.len();

            while i < data_len && memory.dynamic_data[i] != 0 {
                s.push(memory.dynamic_data[i] as char);
                i += 1;
            }

            Ok(s)
        }

    // stack
    } else {
        let mut i = (STACK_SEGMENT - index) as usize;
        let mut s = String::new();
        let stack_len = memory.stack.len();

        while i < stack_len && memory.stack[i] != 0 {
            s.push(memory.stack[i] as char);
            i += 1;
        }

        Ok(s)
    }
}

fn reset(memory: &mut Memory, tokens: &mut Tokens) {
    memory.registers = Registers::default();
    memory.hi = 0;
    memory.lo = 0;
    memory.static_data.clear();
    memory.stack.clear();
    tokens.init();
}

/// Push to data: &Vec<u8> from .data segment's data
fn data_analysis(tokens: &mut Tokens, data: &mut Vec<u8>) {
    let old_idx = tokens.idx();
    tokens.goto(if old_idx == 0 {0} else {old_idx-1});

    // Check all tokens
    'outer: while tokens.consume().is_some() {
        match tokens.token[tokens.idx()].kind {
            TokenKind::INDICATE(IndicateKind::data) => {
                tokens.data_area_now = true;
                continue;
            },
            TokenKind::INDICATE(IndicateKind::text) => {
                tokens.data_area_now = false;
                continue;
            },
            TokenKind::INDICATE(IndicateKind::align(n)) => {
                let padding = 2i32.pow(n as u32) as usize;
                let i = data.len() % padding;
                for _ in 0..i {
                    data.push(0);
                }
                continue;
            },
            _ => (),
        }

        // Ignore except .data segment
        if tokens.data_area_now {
            while {
                // consume EOL
                while TokenKind::EOL == *tokens.kind() {
                    if tokens.next().is_some() {
                        tokens.consume();
                    } else {
                        break 'outer;
                    }
                }

                // until .text segment
                if TokenKind::INDICATE(IndicateKind::text) == *tokens.kind() {
                    tokens.data_area_now = false;
                    continue 'outer;
                }

                // TokenKind::LABEL(usize) = data.len() + 1
                if let TokenKind::LABEL(_, _, ref mut index) = &mut tokens.kind() {
                    *index = Some(data.len()+1);
                    if tokens.next().is_some() && tokens.next().unwrap().kind == TokenKind::EOL {
                        continue 'outer;
                    }
                }

                match &*tokens.kind() {
                    // Align 2^n
                    TokenKind::INDICATE(IndicateKind::align(n)) => {
                        let padding = 2i32.pow(*n as u32) as usize;
                        let i = padding - data.len() % padding;
                        for _ in 0..i {
                            data.push(0);
                        }
                        continue 'outer;
                    },
                    TokenKind::INDICATE(IndicateKind::space(s)) => {
                        for _ in 0..*s {
                            data.push(0);
                        }
                        continue 'outer;
                    },
                    TokenKind::INDICATE(IndicateKind::ascii(s)) => {
                        for ch in s.bytes() {
                            data.push(ch);
                        }
                        continue 'outer;
                    },
                    TokenKind::INDICATE(IndicateKind::asciiz(s)) => {
                        for ch in s.bytes() {
                            data.push(ch);
                        }
                        data.push(0);
                        continue 'outer;
                    },
                    _ => (),
                }

                // until EOL
                while {
                    let still_indicate = match tokens.kind() {
                        // Big Endian
                        TokenKind::INDICATE(IndicateKind::word(w)) => {
                            data.push((*w>>24) as u8);
                            data.push((*w>>16) as u8);
                            data.push((*w>> 8) as u8);
                            data.push( *w      as u8);
                            true
                        },
                        TokenKind::INDICATE(IndicateKind::half(h)) => {
                            data.push((*h>> 8) as u8);
                            data.push( *h      as u8);
                            true
                        },
                        TokenKind::INDICATE(IndicateKind::byte(b)) => {
                            data.push(*b);
                            true
                        },
                        TokenKind::INDICATE(IndicateKind::float(f)) => {
                            data.push((f.to_bits()>>24) as u8);
                            data.push((f.to_bits()>>16) as u8);
                            data.push((f.to_bits()>> 8) as u8);
                            data.push( f.to_bits()      as u8);
                            true
                        },
                        _ => false,
                    };

                    still_indicate && tokens.consume().is_some()
                } {}

                tokens.consume().is_some()
            } {}
        }
    }

    tokens.goto(old_idx);
}

