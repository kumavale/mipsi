use super::super::token::*;
use super::super::token::register::Registers;
use super::super::token::memory::*;
use super::super::parser::{SignExtension, get_int, get_string};

use std::io::Write;
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn eval_arithmetic<F>(registers: &mut Registers, tokens: &mut Tokens, fun: F)
    -> Result<()>
where
    F: Fn(i32, i32) -> Option<i32>,
{
    tokens.consume().ok_or(CONSUME_ERR)?;
    if let Ok(rd_idx) = tokens.expect_register() {
        registers[rd_idx] = {
            tokens.consume().ok_or(CONSUME_ERR)?;
            let register_idx = tokens.expect_register()?;
            let r1 = registers[register_idx];

            if tokens.next().ok_or(CONSUME_ERR)?.kind != TokenKind::EOL {
                tokens.consume().ok_or(CONSUME_ERR)?;
                let r2 = {
                    if let Ok(register_idx) = tokens.expect_register() {
                        registers[register_idx]
                    } else {
                        tokens.expect_integer()?
                    }
                };
                let result = fun(r1, r2);
                if let Some(res) = result {
                    res
                } else {
                    return Err(format!("panicked at 'arithmetic operation overflowed': {}:{}",
                            tokens.filename(), tokens.token[tokens.idx()].line).into());
                }
            } else {
                // CLO, CLZ
                fun(r1, 0).unwrap()
            }
        };
    }

    Ok(())
}

pub fn eval_arithmetic_hilo(memory: &mut Memory, tokens: &mut Tokens, kind: InstructionKind) -> Result<()> {
    let registers = &mut memory.registers;
    let hi        = &mut memory.hi;
    let lo        = &mut memory.lo;

    tokens.consume().ok_or(CONSUME_ERR)?;
    let rd_idx = tokens.expect_register()?;
    tokens.consume().ok_or(CONSUME_ERR)?;
    let rs_idx = tokens.expect_register()?;

    match kind {
        InstructionKind::DIV => {
            if tokens.next().ok_or(CONSUME_ERR)?.kind != TokenKind::EOL {
                tokens.consume().ok_or(CONSUME_ERR)?;
                let rt_idx = tokens.expect_register()?;
                registers[rd_idx] = registers[rs_idx] / registers[rt_idx];
            } else {
                *lo = (registers[rd_idx] / registers[rs_idx]) as u32;
                *hi = (registers[rd_idx] % registers[rs_idx]) as u32;
            }
        },
        InstructionKind::DIVU => {
            if tokens.next().ok_or(CONSUME_ERR)?.kind != TokenKind::EOL {
                tokens.consume().ok_or(CONSUME_ERR)?;
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
        _ => return Err(format!("eval_arithmetic_hilo(): invalid TokenKind: {:?}", kind).into()),
    }

    Ok(())
}

pub fn eval_constant<F>(registers: &mut Registers, tokens: &mut Tokens, fun: F)
    -> Result<()>
where
    F: Fn(i32) -> i32,
{
    tokens.consume().ok_or(CONSUME_ERR)?;
    let register_idx = tokens.expect_register()?;
    registers[register_idx] = {
        tokens.consume().ok_or(CONSUME_ERR)?;
        let integer = tokens.expect_integer()?;
        fun(integer)
    };

    Ok(())
}

pub fn eval_comparison<F>(registers: &mut Registers, tokens: &mut Tokens, fun: F)
    -> Result<()>
where
    F: Fn(i32, i32) -> bool,
{
    tokens.consume().ok_or(CONSUME_ERR)?;
    if let Ok(rd_idx) = tokens.expect_register() {
        tokens.consume().ok_or(CONSUME_ERR)?;
        if let Ok(rs_idx) = tokens.expect_register() {
            tokens.consume().ok_or(CONSUME_ERR)?;
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

pub fn eval_branch<F>(registers: &mut Registers, tokens: &mut Tokens, fun: F)
    -> Result<bool>
where
    F: Fn(i32, i32) -> bool,
{
    tokens.consume().ok_or(CONSUME_ERR)?;
    if let Ok(rsrc1_idx) = tokens.expect_register() {
        tokens.consume().ok_or(CONSUME_ERR)?;
        if let Ok(rsrc2_idx) = tokens.expect_register() {
            tokens.consume().ok_or(CONSUME_ERR)?;
            if fun(registers[rsrc1_idx], registers[rsrc2_idx]) {
                let idx = tokens.expect_label()?;
                tokens.goto(idx-1);
                return Ok(true);
            }
        } else if let Ok(num) = tokens.expect_integer() {
            tokens.consume().ok_or(CONSUME_ERR)?;
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
pub fn eval_jump(registers: &mut Registers, tokens: &mut Tokens, kind: InstructionKind)
    -> Result<()>
{
    match kind {
        InstructionKind::J => {
            tokens.consume().ok_or(CONSUME_ERR)?;
            let idx = tokens.expect_label()?;
            tokens.goto(idx-1);
        },
        InstructionKind::JAL => {
            tokens.consume().ok_or(CONSUME_ERR)?;
            let idx = tokens.expect_label()?;
            registers[31] = tokens.idx() as i32 + 1;  // $ra
            tokens.goto(idx-1);
        },
        InstructionKind::JR => {
            tokens.consume().ok_or(CONSUME_ERR)?;
            let idx = tokens.expect_register()?;
            tokens.goto(registers[idx] as usize);
        },
        InstructionKind::JALR => {
            tokens.consume().ok_or(CONSUME_ERR)?;
            let rs_idx = tokens.expect_register()?;
            tokens.consume();
            let rd_idx = tokens.expect_register()?;
            registers[rd_idx] = tokens.idx() as i32 + 1;
            tokens.goto(rs_idx-1);
        },
        _ => return Err(format!("eval_jump(): invalid InstructionKind: {:?}", kind).into()),
    }

    Ok(())
}

pub fn eval_load(memory: &mut Memory, tokens: &mut Tokens, byte: usize, se: SignExtension) -> Result<()> {
    tokens.consume().ok_or(CONSUME_ERR)?;
    let register_idx = tokens.expect_register()?;
    tokens.consume().ok_or(CONSUME_ERR)?;
    let idx = if let Ok((r_idx, s_idx)) = tokens.expect_memory() { // data or stack
        memory.registers[r_idx] as u32 + s_idx
    } else if let Ok((r_idx, d_idx)) = tokens.expect_data() {
        d_idx as u32 + memory.registers[r_idx] as u32
    } else {
        tokens.expect_address()? as u32
    };
    memory.registers[register_idx] = get_int(&memory, idx, byte, se)?;

    Ok(())
}

pub fn eval_store(memory: &mut Memory, tokens: &mut Tokens, byte: usize) -> Result<()> {
    tokens.consume().ok_or(CONSUME_ERR)?;
    let register_idx = tokens.expect_register()?;
    tokens.consume().ok_or(CONSUME_ERR)?;
    if let Ok((r_idx, append)) = tokens.expect_memory() {
        let idx = memory.registers[r_idx] as u32 + append;

        // data
        if idx < DYNAMIC_DATA_EXIT {
            if idx < DYNAMIC_DATA{
                // static data
                let index = (idx - if STATIC_DATA <= idx { STATIC_DATA } else { 0 } - 1) as usize;
                for i in 0..byte {
                    memory.static_data[index+i] = (memory.registers[register_idx] >> ((byte-1-i)*8)) as u8;
                }
            } else {
                // dynamic data
                let index = (idx - DYNAMIC_DATA) as usize;
                for i in 0..byte {
                    memory.dynamic_data[index+i] = (memory.registers[register_idx] >> ((byte-1-i)*8)) as u8;
                }
            }

        // stack
        } else {
            let index = (STACK_SEGMENT - idx) as usize;
            if memory.stack.len() <= index+byte {
                memory.stack.resize(index+byte+1, 0);
            }
            for i in 0..byte {
                memory.stack[index+i] = (memory.registers[register_idx] >> ((byte-1-i)*8)) as u8;
            }
        }
    } else if let Ok((r_idx, d_idx)) = tokens.expect_data() {
        let index = memory.registers[r_idx] as usize + d_idx - 1;
        for i in 0..byte {
            memory.static_data[index - if STATIC_DATA as usize <= index { STATIC_DATA as usize } else { 0 } + i]
                = (memory.registers[register_idx] >> ((byte-1-i)*8)) as u8;
        }
    } else {
        let index = tokens.expect_address()? - 1;
        for i in 0..byte {
            memory.static_data[index - if STATIC_DATA as usize <= index { STATIC_DATA as usize } else { 0 } + i]
                = (memory.registers[register_idx] >> ((byte-1-i)*8)) as u8;
        }
    }

    Ok(())
}

pub fn eval_myown(memory: &Memory, tokens: &mut Tokens, kind: InstructionKind) -> Result<()> {
    match kind {
        InstructionKind::PRTN => {
            println!();
        },
        InstructionKind::PRTI => {
            tokens.consume().ok_or(CONSUME_ERR)?;
            if let Ok(r_idx) = tokens.expect_register() {
                print!("{}", memory.registers[r_idx]);
            } else if let Ok(num) = tokens.expect_integer() {
                print!("{}", num);
            } else if let Ok((r_idx, s_idx)) = tokens.expect_memory() { // data or stack
                let idx = memory.registers[r_idx] as u32 + s_idx;
                print!("{}", get_int(&memory, idx, 4, SignExtension::Unsigned)?);
            } else if let Ok((r_idx, d_idx)) = tokens.expect_data() {
                let idx = (memory.registers[r_idx] as usize + d_idx) as u32;
                print!("{}", get_int(&memory, idx, 4, SignExtension::Unsigned)?);
            } else {
                let idx = tokens.expect_address()? as u32;
                print!("{}", get_int(&memory, idx, 4, SignExtension::Unsigned)?);
            }
            let _ = std::io::stdout().flush();
        },
        InstructionKind::PRTH => {
            tokens.consume().ok_or(CONSUME_ERR)?;
            if let Ok(r_idx) = tokens.expect_register() {
                print!("{:x}", memory.registers[r_idx]);
            } else if let Ok(num) = tokens.expect_integer() {
                print!("{:x}", num);
            } else if let Ok((r_idx, s_idx)) = tokens.expect_memory() { // data or stack
                let idx = memory.registers[r_idx] as u32 + s_idx;
                print!("{:x}", get_int(&memory, idx, 4, SignExtension::Unsigned)?);
            } else if let Ok((r_idx, d_idx)) = tokens.expect_data() {
                let idx = memory.registers[r_idx] as u32 + d_idx as u32;
                print!("{:x}", get_int(&memory, idx, 4, SignExtension::Unsigned)?);
            } else {
                let idx = tokens.expect_address()? as u32;
                print!("{:x}", get_int(&memory, idx, 4, SignExtension::Unsigned)?);
            }
            let _ = std::io::stdout().flush();
        },
        InstructionKind::PRTX => {
            tokens.consume().ok_or(CONSUME_ERR)?;
            if let Ok(r_idx) = tokens.expect_register() {
                print!("0x{:x}", memory.registers[r_idx]);
            } else if let Ok(num) = tokens.expect_integer() {
                print!("0x{:x}", num);
            } else if let Ok((r_idx, s_idx)) = tokens.expect_memory() { // data or stack
                let idx = memory.registers[r_idx] as u32 + s_idx;
                print!("0x{:x}", get_int(&memory, idx, 4, SignExtension::Unsigned)?);
            } else if let Ok((r_idx, d_idx)) = tokens.expect_data() {
                let idx = memory.registers[r_idx] as u32 + d_idx as u32;
                print!("0x{:x}", get_int(&memory, idx, 4, SignExtension::Unsigned)?);
            } else {
                let idx = tokens.expect_address()? as u32;
                print!("0x{:x}", get_int(&memory, idx, 4, SignExtension::Unsigned)?);
            }
            let _ = std::io::stdout().flush();
        },
        InstructionKind::PRTC => {
            tokens.consume().ok_or(CONSUME_ERR)?;
            if let Ok(r_idx) = tokens.expect_register() {
                print!("{}", memory.registers[r_idx] as u8 as char);
            } else if let Ok(d_idx) = tokens.expect_address() {
                print!("{}", memory.static_data[d_idx-1] as char);
            } else if let Ok((r_idx, s_idx)) = tokens.expect_memory() { // data or stack
                let idx = memory.registers[r_idx] as u32 + s_idx;
                print!("{}", &get_string(&memory, idx)?[..1]);
            } else if let Ok((r_idx, d_idx)) = tokens.expect_data() {
                let idx = (memory.registers[r_idx] as usize + d_idx) as usize;
                print!("{}", memory.static_data[idx-1] as char);
            } else {
                let ch = tokens.expect_integer()? as u8 as char;
                print!("{}", ch);
            }
            let _ = std::io::stdout().flush();
        },
        InstructionKind::PRTS => {
            tokens.consume().ok_or(CONSUME_ERR)?;
            if let Ok(r_idx) = tokens.expect_register() {
                print!("{}", get_string(&memory, memory.registers[r_idx] as u32)?);
            } else if let Ok(d_idx) = tokens.expect_address() {
                print!("{}", get_string(&memory, d_idx as u32)?);
            } else if let Ok((r_idx, s_idx)) = tokens.expect_memory() { // data or stack
                let idx = memory.registers[r_idx] as u32 + s_idx;
                print!("{}", get_string(&memory, idx)?);
            } else if let Ok((r_idx, d_idx)) = tokens.expect_data() {
                let idx = memory.registers[r_idx] as u32 + d_idx as u32;
                print!("{}", get_string(&memory, idx)?);
            } else {
                let s = tokens.expect_literal()?;
                print!("{}", s);
            }
            let _ = std::io::stdout().flush();
        },
        _ => return Err(format!("eval_myown(): invalid TokenKind: {:?}", kind).into()),
    }

    Ok(())
}

