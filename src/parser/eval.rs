use super::super::token::*;
use super::super::parser::{get_int, get_string};

use std::io::Write;

type Result<T> = std::result::Result<T, String>;

pub fn eval_arithmetic<F>(registers: &mut [i32], tokens: &mut Tokens, fun: F)
    -> Result<()>
where
    F: Fn(i32, i32) -> i32,
{
    tokens.consume().unwrap();
    if let Ok(rd_idx) = tokens.expect_register() {
        registers[rd_idx] = {
            tokens.consume().unwrap();
            let register_idx = tokens.expect_register()?;
            let r1 = registers[register_idx];

            if tokens.next().unwrap().kind != TokenKind::EOL {
                tokens.consume().unwrap();
                let r2 = {
                    if let Ok(register_idx) = tokens.expect_register() {
                        registers[register_idx]
                    } else {
                        tokens.expect_integer()?
                    }
                };
                fun(r1, r2)
            } else {
                // CLO, CLZ
                fun(r1, 0)
            }
        };
    }

    Ok(())
}

pub fn eval_arithmetic_hilo(registers: &mut[i32], tokens: &mut Tokens,
    hi: &mut u32, lo: &mut u32, kind: InstructionKind)
    -> Result<()>
{
    tokens.consume().unwrap();
    let rd_idx = tokens.expect_register()?;
    tokens.consume().unwrap();
    let rs_idx = tokens.expect_register()?;

    match kind {
        InstructionKind::DIV => {
            if tokens.next().unwrap().kind != TokenKind::EOL {
                tokens.consume().unwrap();
                let rt_idx = tokens.expect_register()?;
                registers[rd_idx] = registers[rs_idx] / registers[rt_idx];
            } else {
                *lo = (registers[rd_idx] / registers[rs_idx]) as u32;
                *hi = (registers[rd_idx] % registers[rs_idx]) as u32;
            }
        },
        InstructionKind::DIVU => {
            if tokens.next().unwrap().kind != TokenKind::EOL {
                tokens.consume().unwrap();
                let rt_idx = tokens.expect_register()?;
                registers[rd_idx] = (registers[rs_idx] as u32 / registers[rt_idx] as u32) as i32;
            } else {
                *lo = registers[rd_idx] as u32 / registers[rs_idx] as u32;
                *hi = registers[rd_idx] as u32 % registers[rs_idx] as u32;
            }
        },
        InstructionKind::MULT => {
            let ans = registers[rd_idx] as i64 * registers[rs_idx] as i64;
            *lo = ans as u32;
            *hi = ((ans as u64) >> 32) as u32;
        },
        InstructionKind::MULTU => {
            let ans = registers[rd_idx] as u64 * registers[rs_idx] as u64;
            *lo = ans as u32;
            *hi = (ans >> 32) as u32;
        },
        InstructionKind::MADD => {
            let ans = registers[rd_idx] as i64 * registers[rs_idx] as i64;
            *lo += ans as u32;
            *hi += ((ans as u64) >> 32) as u32;
        },
        InstructionKind::MADDU => {
            let ans = registers[rd_idx] as u64 * registers[rs_idx] as u64;
            *lo += ans as u32;
            *hi += (ans >> 32) as u32;
        },
        InstructionKind::MSUB => {
            let ans = registers[rd_idx] as i64 * registers[rs_idx] as i64;
            *lo -= ans as u32;
            *hi -= ((ans as u64) >> 32) as u32;
        },
        InstructionKind::MSUBU => {
            let ans = registers[rd_idx] as u64 * registers[rs_idx] as u64;
            *lo -= ans as u32;
            *hi -= (ans >> 32) as u32;
        },
        _ => return Err(format!("eval_arithmetic_hilo(): invalid TokenKind: {:?}", kind)),
    }

    Ok(())
}

pub fn eval_constant<F>(registers: &mut [i32], tokens: &mut Tokens, fun: F)
    -> Result<()>
where
    F: Fn(i32) -> i32,
{
    tokens.consume().unwrap();
    let register_idx = tokens.expect_register()?;
    registers[register_idx] = {
        tokens.consume().unwrap();
        let integer = tokens.expect_integer()?;
        fun(integer)
    };

    Ok(())
}

pub fn eval_comparison<F>(registers: &mut [i32], tokens: &mut Tokens, fun: F)
    -> Result<()>
where
    F: Fn(i32, i32) -> bool,
{
    tokens.consume().unwrap();
    if let Ok(rd_idx) = tokens.expect_register() {
        tokens.consume().unwrap();
        if let Ok(rs_idx) = tokens.expect_register() {
            tokens.consume().unwrap();
            if let Ok(rt_idx) = tokens.expect_register() {
                registers[rd_idx] = if fun(registers[rs_idx], registers[rt_idx]) {
                    1
                } else {
                    0
                }
            } else {
                let num = tokens.expect_integer()?;
                registers[rd_idx] = if fun(registers[rs_idx], num) {
                    1
                } else {
                    0
                }
            }
        }
    }

    Ok(())
}

pub fn eval_branch<F>(registers: &mut [i32], tokens: &mut Tokens, fun: F)
    -> Result<bool>
where
    F: Fn(i32, i32) -> bool,
{
    tokens.consume().unwrap();
    if let Ok(rsrc1_idx) = tokens.expect_register() {
        tokens.consume().unwrap();
        if let Ok(rsrc2_idx) = tokens.expect_register() {
            tokens.consume().unwrap();
            if fun(registers[rsrc1_idx], registers[rsrc2_idx]) {
                let idx = tokens.expect_label()?;
                tokens.goto(idx-1);
                return Ok(true);
            }
        } else if let Ok(num) = tokens.expect_integer() {
            tokens.consume().unwrap();
            if fun(registers[rsrc1_idx], num) {
                let idx = tokens.expect_label()?;
                tokens.goto(idx-1);
                return Ok(true);
            }
        } else {
            // BEQZ, BGEZ, BGTZ, BLEZ, BLTZ, BNEZ
            let idx = tokens.expect_label()?;
            if fun(registers[rsrc1_idx], 0) {
                tokens.goto(idx-1);
                return Ok(true);
            }
        }
    } else {
        // B // TODO
        let idx = tokens.expect_label()?;
        tokens.goto(idx-1);
        return Ok(true);
    }

    Ok(false)
}

/// Return: can continue
pub fn eval_jump(registers: &mut [i32], tokens: &mut Tokens, kind: InstructionKind)
    -> Result<()>
{
    match kind {
        InstructionKind::J => {
            tokens.consume().unwrap();
            let idx = tokens.expect_label()?;
            tokens.goto(idx-1);
        },
        InstructionKind::JAL => {
            tokens.consume().unwrap();
            let idx = tokens.expect_label()?;
            registers[31] = tokens.idx() as i32 + 1;  // $ra
            tokens.goto(idx-1);
        },
        InstructionKind::JR => {
            tokens.consume().unwrap();
            let idx = tokens.expect_register()?;
            tokens.goto(registers[idx] as usize);
        },
        InstructionKind::JALR => {
            tokens.consume().unwrap();
            let rs_idx = tokens.expect_register()?;
            tokens.consume();
            let rd_idx = tokens.expect_register()?;
            registers[rd_idx] = tokens.idx() as i32 + 1;
            tokens.goto(rs_idx-1);
        },
        _ => return Err(format!("eval_jump(): invalid InstructionKind: {:?}", kind)),
    }

    Ok(())
}

pub fn eval_load(registers: &mut [i32], tokens: &mut Tokens, data: &[u8], stack: &mut Vec<u8>, byte: isize)
    -> Result<()>
{
    tokens.consume().unwrap();
    let register_idx = tokens.expect_register()?;
    tokens.consume().unwrap();
    if let Ok((r_idx, s_idx)) = tokens.expect_memory() { // data or stack
        let idx = registers[r_idx] + s_idx;

        let is_data_idx = if idx < 0 {
            false
        } else if 0 < idx {
            true
        } else {
            return Err("eval_load(): invalid index: 0".to_string());
        };

        // data index
        if is_data_idx {
            let idx = registers[r_idx] as isize;
            registers[register_idx] = get_int(&data, &stack, idx, byte.abs() as usize)?;

        // stack index
        } else {
            let stack_idx = (-idx) as usize;
            if stack.len() <= stack_idx {
                stack.resize(stack_idx+1, 0);
            }
            registers[register_idx] = {
                let int = {
                    if byte < 0 {
                        let mut int: i32 = 0;
                        let byte = byte.abs() as usize;
                        for i in 0..byte {
                            //int |= (stack[stack_idx-(byte-1-i)] as i32) << ((byte-1-i) * 8);
                            int |= (stack[stack_idx+i] as i32) << ((byte-1-i) * 8);
                        }
                        int
                    } else {
                        let mut int: u32 = 0;
                        let byte = byte.abs() as usize;
                        for i in 0..byte {
                            //int |= (stack[stack_idx-(byte-1-i)] as u32) << ((byte-1-i) * 8);
                            int |= (stack[stack_idx+i] as u32) << ((byte-1-i) * 8);
                        }
                        int as i32
                    }
                };
                int
            };
        }
    } else if let Ok((r_idx, d_idx)) = tokens.expect_data() {
        registers[register_idx] = {
            let index = d_idx - 1 + registers[r_idx] as usize;
            let int: i32 = if byte < 0 {
                let mut int: i32 = 0;
                let byte = byte.abs() as usize;
                for i in 0..byte {
                    int |= (data[index+i] as i32) << ((byte-1-i) * 8);
                }
                int
            } else {
                let mut int: u32 = 0;
                let byte = byte.abs() as usize;
                for i in 0..byte {
                    int |= (data[index+i] as u32) << ((byte-1-i) * 8);
                }
                int as i32
            };
            int
        };
    } else {
        let idx = tokens.expect_address()? as isize;
        registers[register_idx] = get_int(&data, &stack, idx, byte.abs() as usize)?;
    }

    Ok(())
}

pub fn eval_myown(registers: &[i32], tokens: &mut Tokens, data: &[u8], stack: &[u8], kind: InstructionKind)
    -> Result<()>
{
    match kind {
        InstructionKind::PRTN => {
            println!();
        },
        InstructionKind::PRTI => {
            tokens.consume().unwrap();
            if let Ok(r_idx) = tokens.expect_register() {
                print!("{}", registers[r_idx]);
            } else if let Ok(num) = tokens.expect_integer() {
                print!("{}", num);
            } else if let Ok((r_idx, s_idx)) = tokens.expect_memory() { // data or stack
                let idx = registers[r_idx] + s_idx;

                let is_data_idx = if idx < 0 {
                    false
                } else if 0 < idx {
                    true
                } else {
                    return Err("eval_myown(): invalid index: 0".to_string());
                };

                // data index
                if is_data_idx {
                    let idx = registers[r_idx] as isize;
                    print!("{}", get_int(&data, &stack, idx, 4)?);

                    // stack index
                } else {
                    let stack_idx = (-idx) as usize;
                    let num = {
                        let mut int = 0;
                        for i in 0..4 {
                            int |= (stack[stack_idx-(4-1-i)] as i32) << ((4-1-i) * 8);
                        }
                        int
                    };
                    print!("{}", num);
                }
            } else {
                let (r_idx, d_idx) = tokens.expect_data()?;
                let num = {
                    let mut int = 0;
                    let index = d_idx - 1 + registers[r_idx] as usize;
                    for i in 0..4 {
                        int |= (data[index+i] as i32) << ((4-1-i) * 8);
                    }
                    int
                };
                print!("{}", num);
            }
            let _ = std::io::stdout().flush();
        },
        InstructionKind::PRTH => {
            tokens.consume().unwrap();
            if let Ok(r_idx) = tokens.expect_register() {
                print!("{:x}", registers[r_idx]);
            } else {
                let num = tokens.expect_integer()?;
                print!("{:x}", num);
            }
            let _ = std::io::stdout().flush();
        },
        InstructionKind::PRTX => {
            tokens.consume().unwrap();
            if let Ok(r_idx) = tokens.expect_register() {
                print!("0x{:x}", registers[r_idx]);
            } else {
                let num = tokens.expect_integer()?;
                print!("0x{:x}", num);
            }
            let _ = std::io::stdout().flush();
        },
        InstructionKind::PRTC => {
            tokens.consume().unwrap();
            if let Ok(r_idx) = tokens.expect_register() {
                print!("{}", &get_string(&data, &stack, registers[r_idx])?[0..1]);
            } else if let Ok(d_idx) = tokens.expect_address() {
                print!("{}", data[d_idx-1] as char);
            } else {
                let ch = tokens.expect_integer()? as u8 as char;
                print!("{}", ch);
            }
            let _ = std::io::stdout().flush();
        },
        InstructionKind::PRTS => {
            tokens.consume().unwrap();
            if let Ok(r_idx) = tokens.expect_register() {
                print!("{}", get_string(&data, &stack, registers[r_idx])?);
            } else if let Ok(d_idx) = tokens.expect_address() {
                print!("{}", get_string(&data, &stack, d_idx as i32)?);
            } else {
                let s = tokens.expect_literal()?;
                print!("{}", s);
            }
            let _ = std::io::stdout().flush();
        },
        _ => return Err(format!("eval_myown(): invalid TokenKind: {:?}", kind)),
    }

    Ok(())
}

