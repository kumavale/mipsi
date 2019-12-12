use super::super::token::*;
use super::super::parser::get_int;

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
            if tokens.consume().is_some() {
                if let Ok(register_idx) = tokens.expect_register() {
                    r2 = registers[register_idx];
                } else if let Ok(num) = tokens.expect_integer() {
                    r2 = num;
                }
            } else {
                // NOT
            }
            fun(r1, r2)
        };
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

