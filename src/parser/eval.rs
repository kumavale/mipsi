use super::super::token::*;
use super::super::parser::{get_int, get_string};

use std::io::Write;

pub fn eval_arithmetic<F>(registers: &mut [i32], tokens: &mut Tokens, fun: F)
where
    F: Fn(i32, i32) -> i32,
{
    tokens.consume().unwrap();
    if let Ok(rd_idx) = tokens.expect_register() {
        registers[rd_idx] = {
            let mut r1 = 0;
            tokens.consume().unwrap();
            if let Ok(register_idx) = tokens.expect_register() {
                r1 = registers[register_idx];
            } else if let Ok(num) = tokens.expect_integer() {
                r1 = num;
            }

            let mut r2 = 0;
            if tokens.next().unwrap().kind != TokenKind::EOL {
                tokens.consume().unwrap();
                if let Ok(register_idx) = tokens.expect_register() {
                    r2 = registers[register_idx];
                } else if let Ok(num) = tokens.expect_integer() {
                    r2 = num;
                }
                fun(r1, r2)
            } else {
                // CLO, CLZ
                fun(r1, 0)
            }
        };
    }
}

pub fn eval_arithmetic_hilo(registers: &mut[i32], tokens: &mut Tokens,
    hi: &mut u32, lo: &mut u32, kind: InstructionKind)
{
    tokens.consume().unwrap();
    let rd_idx = tokens.expect_register().unwrap();
    tokens.consume().unwrap();
    let rs_idx = tokens.expect_register().unwrap();

    match kind {
        InstructionKind::DIV => {
            if tokens.next().unwrap().kind != TokenKind::EOL {
                tokens.consume().unwrap();
                let rt_idx = tokens.expect_register().unwrap();
                registers[rd_idx] = registers[rs_idx] / registers[rt_idx];
            } else {
                *lo = (registers[rd_idx] / registers[rs_idx]) as u32;
                *hi = (registers[rd_idx] % registers[rs_idx]) as u32;
            }
        },
        InstructionKind::DIVU => {
            if tokens.next().unwrap().kind != TokenKind::EOL {
                tokens.consume().unwrap();
                let rt_idx = tokens.expect_register().unwrap();
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
        _ => println!("eval_arithmetic_hilo(): invalid TokenKind: {:?}", kind),
    }
}

pub fn eval_constant<F>(registers: &mut [i32], tokens: &mut Tokens, fun: F)
where
    F: Fn(i32) -> i32,
{
    tokens.consume().unwrap();
    let register_idx = tokens.expect_register().unwrap();
    registers[register_idx] = {
        tokens.consume().unwrap();
        let integer = tokens.expect_integer().unwrap();
        fun(integer)
    };
}

pub fn eval_comparison<F>(registers: &mut [i32], tokens: &mut Tokens, fun: F)
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
                let num = tokens.expect_integer().unwrap();
                registers[rd_idx] = if fun(registers[rs_idx], num) {
                    1
                } else {
                    0
                }
            }
        }
    }
}

pub fn eval_branch<F>(registers: &mut [i32], tokens: &mut Tokens, fun: F) -> bool
where
    F: Fn(i32, i32) -> bool,
{
    tokens.consume().unwrap();
    if let Ok(rsrc1_idx) = tokens.expect_register() {
        tokens.consume().unwrap();
        if let Ok(rsrc2_idx) = tokens.expect_register() {
            tokens.consume().unwrap();
            if fun(registers[rsrc1_idx], registers[rsrc2_idx]) {
                let idx = tokens.expect_label().unwrap();
                tokens.goto(idx-1);
                return true;
            }
        } else if let Ok(num) = tokens.expect_integer() {
            tokens.consume().unwrap();
            if fun(registers[rsrc1_idx], num) {
                let idx = tokens.expect_label().unwrap();
                tokens.goto(idx-1);
                return true;
            }
        } else {
            // BEQZ, BGEZ, BGTZ, BLEZ, BLTZ, BNEZ
            let idx = tokens.expect_label().unwrap();
            if fun(registers[rsrc1_idx], 0) {
                tokens.goto(idx-1);
                return true;
            }
        }
    } else {
        // B // TODO
        let idx = tokens.expect_label().unwrap();
        tokens.goto(idx-1);
        return true;
    }

    false
}

/// Return: can continue
pub fn eval_jump(registers: &mut [i32], tokens: &mut Tokens, kind: InstructionKind) -> bool {
    match kind {
        InstructionKind::J => {
            tokens.consume().unwrap();
            if let Ok(idx) = tokens.expect_label() {
                tokens.goto(idx-1);
                return true;
            }
        },
        InstructionKind::JAL => {
            tokens.consume().unwrap();
            if let Ok(idx) = tokens.expect_label() {
                registers[31] = tokens.idx() as i32 + 1;  // $ra
                tokens.goto(idx-1);
                return true;
            }
        },
        InstructionKind::JR => {
            tokens.consume().unwrap();
            if let Ok(idx) = tokens.expect_register() {
                tokens.goto(registers[idx] as usize);
                return true;
            }
        },
        InstructionKind::JALR => {
            tokens.consume().unwrap();
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

pub fn eval_load(registers: &mut [i32], tokens: &mut Tokens, data: &[u8], stack: &mut Vec<u8>, byte: usize) {
    tokens.consume().unwrap();
    let register_idx = tokens.expect_register().unwrap();
    tokens.consume().unwrap();
    if let Ok((r_idx, s_idx)) = tokens.expect_stack() { // data or stack
        let idx = registers[r_idx] + s_idx;

        let is_data_idx = if idx < 0 {
            false
        } else if 0 < idx {
            true
        } else {
            panic!("eval_load(): invalid index: 0");
        };

        // data index
        if is_data_idx {
            let idx = registers[r_idx] as isize;
            registers[register_idx] = get_int(&data, &stack, idx, byte);

        // stack index
        } else {
            let stack_idx = (-idx) as usize;
            if stack.len() <= stack_idx {
                stack.resize(stack_idx+1, 0);
            }
            registers[register_idx] = {
                let mut int = 0;
                for i in 0..byte {
                    int |= (stack[stack_idx-(byte-1-i)] as i32) << ((byte-1-i) * 8);
                }
                int
            };
        }
    } else if let Ok((r_idx, d_idx)) = tokens.expect_data() {
        registers[register_idx] = {
            let mut int = 0;
            let index = d_idx - 1 + registers[r_idx] as usize;
            for i in 0..byte {
                int |= (data[index+i] as i32) << ((byte-1-i) * 8);
            }
            int
        };
    } else {
        let idx = tokens.expect_address().unwrap() as isize;
        registers[register_idx] = get_int(&data, &stack, idx, byte);
    }
}

pub fn eval_myown(registers: &[i32], tokens: &mut Tokens, data: &[u8], stack: &[u8], kind: InstructionKind) {
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
            } else if let Ok((r_idx, s_idx)) = tokens.expect_stack() { // data or stack
                let idx = registers[r_idx] + s_idx;

                let is_data_idx = if idx < 0 {
                    false
                } else if 0 < idx {
                    true
                } else {
                    panic!("eval_myown(): invalid index: 0");
                };

                // data index
                if is_data_idx {
                    let idx = registers[r_idx] as isize;
                    print!("{}", get_int(&data, &stack, idx, 4));

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
                let (r_idx, d_idx) = tokens.expect_data().unwrap();
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
            std::io::stdout().flush().unwrap();
        },
        InstructionKind::PRTH => {
            tokens.consume().unwrap();
            if let Ok(r_idx) = tokens.expect_register() {
                print!("{:x}", registers[r_idx]);
            } else {
                let num = tokens.expect_integer().unwrap();
                print!("{:x}", num);
            }
            std::io::stdout().flush().unwrap();
        },
        InstructionKind::PRTX => {
            tokens.consume().unwrap();
            if let Ok(r_idx) = tokens.expect_register() {
                print!("0x{:x}", registers[r_idx]);
            } else {
                let num = tokens.expect_integer().unwrap();
                print!("0x{:x}", num);
            }
            std::io::stdout().flush().unwrap();
        },
        InstructionKind::PRTC => {
            tokens.consume().unwrap();
            if let Ok(r_idx) = tokens.expect_register() {
                print!("{}", &get_string(&data, &stack, registers[r_idx])[0..1]);
            } else if let Ok(d_idx) = tokens.expect_address() {
                print!("{}", data[d_idx-1] as char);
            } else {
                let ch = tokens.expect_integer().unwrap() as u8 as char;
                print!("{}", ch);
            }
            std::io::stdout().flush().unwrap();
        },
        InstructionKind::PRTS => {
            tokens.consume().unwrap();
            if let Ok(r_idx) = tokens.expect_register() {
                print!("{}", get_string(&data, &stack, registers[r_idx]));
            } else if let Ok(d_idx) = tokens.expect_address() {
                print!("{}", get_string(&data, &stack, d_idx as i32));
            } else {
                let s = tokens.expect_literal().unwrap();
                print!("{}", s);
            }
            std::io::stdout().flush().unwrap();
        },
        _ => println!("eval_myown(): invalid TokenKind: {:?}", kind),
    }
}

