extern crate rand;
use rand::Rng;

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
        while match *tokens.kind() {
            TokenKind::LABEL(_, _, _) | TokenKind::INDICATE(_) | TokenKind::EOL => true,
            _ => false,
        } { if tokens.consume().is_none() { return Ok(()); } }

        let instruction_kind = tokens.expect_instruction()?;

        match instruction_kind {
            // Arithmetic, Logic
            InstructionKind::ADD |
            InstructionKind::ADDI =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| x.checked_add(y))?,
            InstructionKind::ADDU |
            InstructionKind::ADDIU =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| Some(x + y))?,
            InstructionKind::SUB =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| x.checked_sub(y))?,
            InstructionKind::SUBU =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| Some(x - y))?,
            InstructionKind::MUL =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| x.checked_mul(y))?, // TODO: mult $2,$3;mflo $1
            InstructionKind::REM =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| Some(x % y))?,
            InstructionKind::REMU =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| x.checked_rem(y))?,

            InstructionKind::DIV =>
                eval_arithmetic_hilo(&mut registers, &mut tokens, &mut hi, &mut lo, InstructionKind::DIV)?,
            InstructionKind::DIVU =>
                eval_arithmetic_hilo(&mut registers, &mut tokens, &mut hi, &mut lo, InstructionKind::DIVU)?,
            InstructionKind::MULT =>
                eval_arithmetic_hilo(&mut registers, &mut tokens, &mut hi, &mut lo, InstructionKind::MULT)?,
            InstructionKind::MULTU =>
                eval_arithmetic_hilo(&mut registers, &mut tokens, &mut hi, &mut lo, InstructionKind::MULTU)?,
            InstructionKind::MADD =>
                eval_arithmetic_hilo(&mut registers, &mut tokens, &mut hi, &mut lo, InstructionKind::MADD)?,
            InstructionKind::MADDU =>
                eval_arithmetic_hilo(&mut registers, &mut tokens, &mut hi, &mut lo, InstructionKind::MADDU)?,
            InstructionKind::MSUB =>
                eval_arithmetic_hilo(&mut registers, &mut tokens, &mut hi, &mut lo, InstructionKind::MSUB)?,
            InstructionKind::MSUBU =>
                eval_arithmetic_hilo(&mut registers, &mut tokens, &mut hi, &mut lo, InstructionKind::MSUBU)?,

            InstructionKind::MULO =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| Some(x * y))?,
            InstructionKind::MULOU =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| Some((x as u32 * y as u32) as i32))?,
            InstructionKind::CLO =>
                eval_arithmetic(&mut registers, &mut tokens, move |x, _| {
                    let mut cnt: i32 = 0;
                    for i in (0..=31).rev() {
                        if (x as usize) >> i & 1 != 1 { break; }
                        cnt += 1;
                    }
                    Some(cnt)
                })?,
            InstructionKind::CLZ =>
                eval_arithmetic(&mut registers, &mut tokens, move |x, _| {
                    let mut cnt: i32 = 0;
                    for i in (0..=31).rev() {
                        if (x as usize) >> i & 1 != 0 { break; }
                        cnt += 1;
                    }
                    Some(cnt)
                })?,
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
                            return Err("ROR: invalid token".to_string());
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
                            return Err("ROL: invalid token".to_string());
                        }
                    };
                    registers[1] = rs << rt;
                    registers[rd_idx] = (rs as u32 >> (32-rt)) as i32;
                    registers[rd_idx] | registers[1]
                };
            },

            InstructionKind::NOR =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| Some(!(x | y)))?,
            InstructionKind::NOT => {
                tokens.consume().unwrap();
                let rd_idx = tokens.expect_register()?;
                registers[rd_idx] = {
                    tokens.consume().unwrap();
                    let register_idx = tokens.expect_register()?;
                    !registers[register_idx]
                };
            },
            InstructionKind::NEG =>
                eval_arithmetic(&mut registers, &mut tokens, |x, _| Some(-x))?, // TODO (with overflow)
            InstructionKind::NEGU =>
                eval_arithmetic(&mut registers, &mut tokens, |x, _| Some(-x))?,

            InstructionKind::SLL |
            InstructionKind::SLLV =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| Some(x << y))?,
            InstructionKind::SRA |
            InstructionKind::SRAV =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| Some(x >> y))?,
            InstructionKind::SRL |
            InstructionKind::SRLV =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| Some((x as u32 >> y) as i32))?,

            InstructionKind::AND |
            InstructionKind::ANDI =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| Some(x & y))?,
            InstructionKind::OR |
            InstructionKind::ORI =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| Some(x | y))?,
            InstructionKind::XOR |
            InstructionKind::XORI =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| Some(x ^ y))?,

            // Constant
            InstructionKind::LI =>
                eval_constant(&mut registers, &mut tokens, |x| x)?,
            InstructionKind::LUI =>
                eval_constant(&mut registers, &mut tokens, |x| x & (std::u32::MAX-65535) as i32)?,

            // Comparison
            InstructionKind::SLT |
            InstructionKind::SLTI =>
                eval_comparison(&mut registers, &mut tokens, |x, y| x < y)?,
            InstructionKind::SEQ =>
                eval_comparison(&mut registers, &mut tokens, |x, y| x == y)?,
            InstructionKind::SGE =>
                eval_comparison(&mut registers, &mut tokens, |x, y| x >= y)?,
            InstructionKind::SGT =>
                eval_comparison(&mut registers, &mut tokens, |x, y| x > y)?,
            InstructionKind::SLE =>
                eval_comparison(&mut registers, &mut tokens, |x, y| x <= y)?,
            InstructionKind::SNE =>
                eval_comparison(&mut registers, &mut tokens, |x, y| x != y)?,

            // Branch
            InstructionKind::B =>
                if eval_branch(&mut registers, &mut tokens, |_, _| true)?   { continue; },
            InstructionKind::BEQ =>
                if eval_branch(&mut registers, &mut tokens, |x, y| x == y)? { continue; },
            InstructionKind::BNE =>
                if eval_branch(&mut registers, &mut tokens, |x, y| x != y)? { continue; },
            InstructionKind::BGE =>
                if eval_branch(&mut registers, &mut tokens, |x, y| x >= y)? { continue; },
            InstructionKind::BGT =>
                if eval_branch(&mut registers, &mut tokens, |x, y| x > y)?  { continue; },
            InstructionKind::BLE =>
                if eval_branch(&mut registers, &mut tokens, |x, y| x <= y)? { continue; },
            InstructionKind::BLT =>
                if eval_branch(&mut registers, &mut tokens, |x, y| x < y)?  { continue; },
            InstructionKind::BEQZ =>
                if eval_branch(&mut registers, &mut tokens, |x, y| x == y)? { continue; },
            InstructionKind::BGEZ =>
                if eval_branch(&mut registers, &mut tokens, |x, y| x >= y)? { continue; },
            InstructionKind::BGTZ =>
                if eval_branch(&mut registers, &mut tokens, |x, y| x > y)?  { continue; },
            InstructionKind::BLEZ =>
                if eval_branch(&mut registers, &mut tokens, |x, y| x <= y)? { continue; },
            InstructionKind::BLTZ =>
                if eval_branch(&mut registers, &mut tokens, |x, y| x < y)?  { continue; },
            InstructionKind::BNEZ =>
                if eval_branch(&mut registers, &mut tokens, |x, y| x != y)? { continue; },
            InstructionKind::BGEZAL => {
                tokens.consume().unwrap();
                let r_idx = tokens.expect_register()?;
                tokens.consume().unwrap();
                let l_idx = tokens.expect_label()?;
                registers[31] = tokens.idx() as i32 + 1;  // $ra
                if 0 <= registers[r_idx] {
                    tokens.goto(l_idx-1);
                }
            },
            InstructionKind::BLTZAL => {
                tokens.consume().unwrap();
                let r_idx = tokens.expect_register()?;
                tokens.consume().unwrap();
                let l_idx = tokens.expect_label()?;
                registers[31] = tokens.idx() as i32 + 1;  // $ra
                if registers[r_idx] < 0 {
                    tokens.goto(l_idx-1);
                }
            },

            // Jump
            InstructionKind::J =>
                { eval_jump(&mut registers, &mut tokens, InstructionKind::J)?;    continue; },
            InstructionKind::JAL =>
                { eval_jump(&mut registers, &mut tokens, InstructionKind::JAL)?;  continue; },
            InstructionKind::JR =>
                { eval_jump(&mut registers, &mut tokens, InstructionKind::JR)?;   continue; },
            InstructionKind::JALR =>
                { eval_jump(&mut registers, &mut tokens, InstructionKind::JALR)?; continue; },

            // Load
            InstructionKind::LA => {
                tokens.consume().unwrap();
                let register_idx = tokens.expect_register()?;
                tokens.consume().unwrap();
                registers[register_idx] = {
                    if let Ok(data_idx) = tokens.expect_address() {
                        data_idx as i32
                    } else {
                        tokens.expect_label().unwrap() as i32
                    }
                };
            },
            InstructionKind::LB =>   // Rt = *((int*)address) (8bit)
                eval_load(&mut registers, &mut tokens, &data, &stack, 1, SignExtension::Signed)?,
            InstructionKind::LBU =>  // Rt = *((int*)address) (8bit)
                eval_load(&mut registers, &mut tokens, &data, &stack, 1, SignExtension::Unsigned)?,
            InstructionKind::LH =>   // Rt = *((int*)address) (16bit)
                eval_load(&mut registers, &mut tokens, &data, &stack, 2, SignExtension::Signed)?,
            InstructionKind::LHU =>  // Rt = *((int*)address) (16bit)
                eval_load(&mut registers, &mut tokens, &data, &stack, 2, SignExtension::Unsigned)?,
            InstructionKind::LW =>   // Rt = *((int*)address) (32bit)
                eval_load(&mut registers, &mut tokens, &data, &stack, 4, SignExtension::Unsigned)?,

            // Store
            InstructionKind::SB =>  // *((int*)address) = Rt (8bit)
                eval_store(&mut registers, &mut tokens, &mut data, &mut stack, 1)?,
            InstructionKind::SH =>  // *((int*)address) = Rt (16bit)
                eval_store(&mut registers, &mut tokens, &mut data, &mut stack, 2)?,
            InstructionKind::SW =>  // *((int*)address) = Rt (32bit)
                eval_store(&mut registers, &mut tokens, &mut data, &mut stack, 4)?,

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
                        return Err("TODO".to_string());
                    };
                    registers[r1_idx]
                };
            },
            InstructionKind::MFHI => {
                tokens.consume().unwrap();
                let r_idx = tokens.expect_register()?;
                registers[r_idx] = *hi as i32;
            },
            InstructionKind::MFLO => {
                tokens.consume().unwrap();
                let r_idx = tokens.expect_register()?;
                registers[r_idx] = *lo as i32;
            },
            InstructionKind::MTHI => {
                tokens.consume().unwrap();
                let r_idx = tokens.expect_register()?;
                *hi = registers[r_idx] as u32;
            },
            InstructionKind::MTLO => {
                tokens.consume().unwrap();
                let r_idx = tokens.expect_register()?;
                *lo = registers[r_idx] as u32;
            },
            InstructionKind::MOVN => {
                tokens.consume().unwrap();
                let rd_idx = tokens.expect_register()?;
                tokens.consume().unwrap();
                let rs_idx = tokens.expect_register()?;
                tokens.consume().unwrap();
                let rt_idx = tokens.expect_register()?;
                if registers[rt_idx] != 0 {
                    registers[rd_idx] = registers[rs_idx];
                }
            },
            InstructionKind::MOVZ => {
                tokens.consume().unwrap();
                let rd_idx = tokens.expect_register()?;
                tokens.consume().unwrap();
                let rs_idx = tokens.expect_register()?;
                tokens.consume().unwrap();
                let rt_idx = tokens.expect_register()?;
                if registers[rt_idx] == 0 {
                    registers[rd_idx] = registers[rs_idx];
                }
            },

            // Exception, Interrupt
            InstructionKind::SYSCALL => {
                match registers[2] {  // v0
                    // print_int: $a0=integer
                    1  => {
                        print!("{}", registers[4]);  // $a0
                        let _ = std::io::stdout().flush();
                    },
                    // print_string: $a0=string(data index)
                    4  => {
                        print!("{}", get_string(&data, &stack, registers[4])?);  // $a0
                        let _ = std::io::stdout().flush();
                    },
                    // read_int: return $v0
                    5  => {
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();
                        registers[2] = if let Ok(num) = input.trim().parse::<i32>() {
                            num
                        } else {
                            0
                        };
                    },
                    // read_string: $a0=buffer, $a1=length.  write buffer
                    8  => {
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();
                        let mut index = registers[4] as usize - 1;
                        if data.len() < index + input.len() {
                            return Err(format!("not enough space for .data: {}", registers[4]));
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
                        reset(&mut registers, &mut hi, &mut lo, &mut data, &mut stack, &mut tokens);
                        break;
                    },
                    // print_character
                    11 => {
                        print!("{}", registers[4] as u8 as char);
                        let _ = std::io::stdout().flush();
                    },
                    // read character
                    12 => {
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();
                        registers[2] = input.as_bytes()[0] as i32;
                    },
                    // exit2
                    17 => {
                        std::process::exit(registers[4]);
                    },
                    // random_int:
                    // $a0 = random number(int)
                    41 => {
                        let rnd = rand::thread_rng().gen();
                        registers[4] = rnd;
                    },
                    // random_int_range:
                    // $a0 = random number(int)
                    // $a1 = upper bound of range of returned valus.
                    42 => {
                        let rnd = rand::thread_rng().gen_range(0, registers[5]);  // $a1
                        registers[4] = rnd;
                    },
                    _ => return Err(format!("SYSCALL: invalid code: {}", registers[2])),
                }
            },
            InstructionKind::NOP => (),  // Do nothing

            // My own
            InstructionKind::PRTN =>
                eval_myown(&registers, &mut tokens, &data, &stack, InstructionKind::PRTN)?,
            InstructionKind::PRTI =>
                eval_myown(&registers, &mut tokens, &data, &stack, InstructionKind::PRTI)?,
            InstructionKind::PRTH =>
                eval_myown(&registers, &mut tokens, &data, &stack, InstructionKind::PRTH)?,
            InstructionKind::PRTX =>
                eval_myown(&registers, &mut tokens, &data, &stack, InstructionKind::PRTX)?,
            InstructionKind::PRTC =>
                eval_myown(&registers, &mut tokens, &data, &stack, InstructionKind::PRTC)?,
            InstructionKind::PRTS =>
                eval_myown(&registers, &mut tokens, &data, &stack, InstructionKind::PRTS)?,
            InstructionKind::RST => {
                reset(&mut registers, &mut hi, &mut lo, &mut data, &mut stack, &mut tokens);
                break;
            },
            //_ => (),
        }

        // expect TokenKind::EOL
        tokens.consume();
        tokens.expect_eol()?;

        if tokens.data_trace() {
            display_data_per_4byte(&data);
        }
        if tokens.stack_trace() {
            display_stack(&stack);
        }
        if tokens.register_trace() {
            display_register(&registers);
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
/// ```rust
/// let int: i32 = get_int(&data, &stack, registers[4], 4, SignExtension::Signed)?;
/// ```
///
/// argument1: data:&[u8]
/// argument2: stack:&[u8]
/// argument3: index: isize  =>  stack(<=0) | data(0<)
/// argument4: byte
pub fn get_int(data: &[u8], stack: &[u8], index: isize, byte: usize, se: SignExtension)
    -> Result<i32, String>
{
    let mut int: u32 = 0;

    // data
    if 0 < index {
        let index = (index - 1) as usize;
        if data.len() < index+byte {
            return Err(
                format!("get_int(): index out of bounds: the data len is {}, but the index is {}-{}",
                data.len(), index, index+byte-1));
        }
        // Big Endian
        for i in 0..byte {
            int |= (data[index+i] as u32) << ((byte-1-i) * 8);
        }

    // stack
    } else {
        let index = -index as usize;
        if stack.len() < index+byte {
            return Err(
                format!("get_int(): index out of bounds: the stack len is {}, but the index is {}-{}",
                stack.len(), index, index+byte-1));
        }
        // Big Endian
        for i in 0..byte {
            int |= (stack[index+i] as u32) << ((byte-1-i) * 8);
        }
    }

    match se {
        SignExtension::Signed   => Ok(-(int as i32)),
        SignExtension::Unsigned => Ok(  int as i32),
    }
}

pub fn get_string(data: &[u8], stack: &[u8], index: i32)
    -> Result<String, String>
{
    // data
    if 0 < index {
        let mut i = (index - 1) as usize;
        let mut s = String::new();
        let data_len = data.len();

        while i < data_len && data[i] != 0 {
            s = format!("{}{}", s, data[i] as char);
            i += 1;
        }

        Ok(s)

    // stack
    } else {
        let mut i = -index as usize;
        let mut s = String::new();
        let stack_len = stack.len();

        while i < stack_len && stack[i] != 0 {
            s = format!("{}{}", s, stack[i] as char);
            i += 1;
        }

        Ok(s)
    }
}

fn reset(registers: &mut &mut[i32], hi: &mut &mut u32, lo: &mut &mut u32,
    data: &mut &mut Vec<u8>, stack: &mut &mut Vec<u8>, tokens: &mut &mut Tokens) {
    for r in registers.iter_mut() { *r = 0; }
    **hi = 0;
    **lo = 0;
    data.clear();
    stack.clear();
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
                    if tokens.next().is_some()
                        && tokens.next().unwrap().kind == TokenKind::EOL {
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

