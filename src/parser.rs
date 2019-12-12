use std::io::Write;

use super::token::*;


pub fn parse(mut tokens: Tokens) {

    let mut registers: [i32; 32] = [0; 32];
    let mut addresses: [Option<i32>; 32] = [None; 32];  // TODO delete

    // let **registers = { &zero, &at, ...};

    let mut data:  Vec<u8> = Vec::new();
    let mut stack: Vec<u8> = vec![0];

    data_analysis(&mut tokens, &mut data);
    println!("data: {:?}", data);

    #[allow(unused)]
    while let Some(token) = tokens.consume() {
        //println!("{:?}", token); continue;

        //// Skip LABEL, INDICATE and EOL
        if let TokenKind::LABEL(_, _, _) = tokens.kind() {
            tokens.consume().unwrap();
            if tokens.expect_eol().is_ok() { continue; }
        }
        if let TokenKind::INDICATE(_) = tokens.kind() {
            while let TokenKind::INDICATE(_) = tokens.consume().unwrap().kind { continue; }
            if tokens.expect_eol().is_ok() { continue; }
        }

        let instruction_kind = tokens.expect_instruction().unwrap();

        match instruction_kind {
            // Arithmetic, Logic
            InstructionKind::ADD  |
            InstructionKind::ADDI =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| x + y),
            InstructionKind::SUB =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| x - y),
            InstructionKind::MUL =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| x * y),
            InstructionKind::DIV =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| x / y),
            InstructionKind::REM =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| x % y),

            InstructionKind::NOR =>
                eval_arithmetic(&mut registers, &mut tokens, |x, y| !(x | y)),
            InstructionKind::NOT =>
                eval_arithmetic(&mut registers, &mut tokens, |x, _| !x),

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
                eval_constant(&mut registers, &mut tokens, |x| x & (std::i32::MAX - 65535)),

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

            // Load, Store
            InstructionKind::LA => {
                tokens.consume().unwrap();
                let register_idx = tokens.expect_register().unwrap();
                tokens.consume().unwrap();
                let label_idx = tokens.expect_address().unwrap() as i32;
                registers[register_idx] = label_idx;
                addresses[register_idx] = Some(label_idx);
            },
            InstructionKind::LW => {  // Rt = *((int*)address) (32bit)
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
                        panic!("TODO");
                    };

                    // data index
                    if is_data_idx {
                        let idx = registers[r_idx] as isize;
                        registers[register_idx] = get_int(&data, &stack, idx);

                    // stack index
                    } else {
                        let stack_idx = (-idx) as usize;
                        if stack.len() <= stack_idx {
                            stack.resize(stack_idx+1, 0);
                        }
                        registers[register_idx] = {
                            let mut int = 0;
                            int |= (stack[stack_idx-3] as i32) << 24;
                            int |= (stack[stack_idx-2] as i32) << 16;
                            int |= (stack[stack_idx-1] as i32) <<  8;
                            int |= (stack[stack_idx-0] as i32);
                            int
                        };
                    }
                } else {
                    let idx = tokens.expect_address().unwrap() as isize;
                    registers[register_idx] = get_int(&data, &stack, idx);
                }
            },
            InstructionKind::SW => {  // *((int*)address) = Rt (32bit)
                tokens.consume().unwrap();
                let register_idx = tokens.expect_register().unwrap();
                tokens.consume().unwrap();
                let (r_idx, s_idx) = tokens.expect_stack().unwrap();
                let stack_idx = -(registers[r_idx] + s_idx) as usize;
                if stack.len() <= stack_idx {
                    stack.resize(stack_idx+1, 0);
                }
                stack[stack_idx-3] = (registers[register_idx]>>24) as u8;
                stack[stack_idx-2] = (registers[register_idx]>>16) as u8;
                stack[stack_idx-1] = (registers[register_idx]>> 8) as u8;
                stack[stack_idx-0] = (registers[register_idx]    ) as u8;
            },

            // Transfer
            InstructionKind::MOVE => {
                tokens.consume().unwrap();
                let register_idx = tokens.expect_register().unwrap();
                registers[register_idx] = {
                    let r1_idx = if tokens.consume().is_some() {
                        tokens.expect_register().unwrap()
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
                    // print_string: $a0=string(label)
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
                    10 => break,

                    // My define
                    // print_int + '\n'
                    128 => println!("{}", registers[4]),  // $a0
                    // print_int(unsigned)
                    //129 => println!("{}", registers[4]),  // $a0
                    // print_int + '\n'
                    //130 => println!("{}", registers[4]),  // $a0
                    // read_char (without enter)
                    //131 => (),

                    _ => println!("SYSCALL: invalid code: {}", registers[2]),
                }
            },
            InstructionKind::NOP => (),  // Do nothing
            _ => (),
        }

        // expect TokenKind::EOL
        tokens.consume();
        tokens.expect_eol().unwrap();

        if std::env::var("STACK_TRACE").is_ok() {
            display_stack(&stack);
        }
        if std::env::var("REGISTER_TRACE").is_ok() {
            display_register(&registers);
        }
    }

    println!("\ndata: {:?}", data);
    display_register(&registers);
}

// Display all registers
fn display_register(registers: &[i32]) {
    println!("\n================================================================");
    for i in 0..8 {
        for j in 0..4 {
            if registers[i+j*8] == 0 {
                print!(" ${:<2}:0x{:08x}\t", i+j*8, registers[i+j*8]);
            } else {
                print!(" ${:<2}:\x1b[31m0x{:08x}\x1b[m\t", i+j*8, registers[i+j*8]);
            }
            std::io::stdout().flush().unwrap();
        }
        println!();
    }
    println!("================================================================");
}

fn display_stack(stack: &Vec<u8>) {
    println!("stack: {:?}", stack);
}

fn eval_arithmetic<F>(registers: &mut [i32], tokens: &mut Tokens, fun: F)
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

fn eval_constant<F>(registers: &mut [i32], tokens: &mut Tokens, fun: F)
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

fn eval_comparison<F>(registers: &mut [i32], tokens: &mut Tokens, fun: F)
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

fn eval_branch<F>(registers: &mut [i32], tokens: &mut Tokens, fun: F) -> bool
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
fn eval_jump(registers: &mut [i32], tokens: &mut Tokens, kind: InstructionKind) -> bool {
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
    tokens.push(TokenKind::LABEL("loop".to_string(), 30), 31, None);
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
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::SW), 54);
    tokens.push(TokenKind::REGISTER(RegisterKind::t4, 12), 55);
    tokens.push(TokenKind::STACK(RegisterKind::sp, 29, 0), 56);
    tokens.push(TokenKind::EOL, 57);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::SLT), 58);
    tokens.push(TokenKind::REGISTER(RegisterKind::t0,  8), 59);
    tokens.push(TokenKind::REGISTER(RegisterKind::t7, 15), 60);
    tokens.push(TokenKind::REGISTER(RegisterKind::v0,  2), 61);
    tokens.push(TokenKind::EOL, 62);
    tokens.push(TokenKind::INSTRUCTION(InstructionKind::NOT), 63);
    tokens.push(TokenKind::REGISTER(RegisterKind::t7, 15), 64);
    tokens.push(TokenKind::REGISTER(RegisterKind::v0,  2), 65);
    tokens.push(TokenKind::EOL, 66);

    parse(tokens);
}

/// Return signed integer (32-bit)
///
/// # Example
///
/// ```rust
/// let int: i32 = get_int(&data, &stack, registers[4]);
/// ```
///
//// argument1: memory: &<T>  =>  registers:&[i32] | stack:&Vec<u8> | data:&Vec<u8>
/// argument1: data:&Vec<u8>
/// argument2: stack:&Vec<u8>
/// argument3: index: isize  =>  stack(-) | data(+)
pub fn get_int(data: &Vec<u8>, stack: &Vec<u8>, index: isize) -> i32 {

    // stack
    if index < 0 {
        let index = (-index - 1) as usize;
        let octet1 = stack[index+0] as i32;
        let octet2 = stack[index+1] as i32;
        let octet3 = stack[index+2] as i32;
        let octet4 = stack[index+3] as i32;

        // Big Endian
        let mut int: i32 = 0;
        int |= octet1 << 24;
        int |= octet2 << 16;
        int |= octet3 <<  8;
        int |= octet4;

        int

    // data
    } else if 0 < index {
        let index = ( index - 1) as usize;
        let octet1 = data[index+0] as i32;
        let octet2 = data[index+1] as i32;
        let octet3 = data[index+2] as i32;
        let octet4 = data[index+3] as i32;

        // Big Endian
        let mut int: i32 = 0;
        int |= octet1 << 24;
        int |= octet2 << 16;
        int |= octet3 <<  8;
        int |= octet4;

        int

    } else {
        panic!(format!("get_int(): invalid index: {}", index));
    }
}

pub fn get_string(data: &Vec<u8>, stack: &Vec<u8>, index: i32) -> String {
    // stack
    if index < 0 {
        let mut i = (-index - 1) as usize;
        let mut s = String::new();

        while stack[i] != 0 {
            s = format!("{}{}", s, stack[i] as char);
            i += 1;
        }
        s

    // data
    } else if 0 < index {
        let mut i = (index - 1) as usize;
        let mut s = String::new();

        while data[i] != 0 {
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

    // Check all tokens
    while tokens.consume().is_some() {

        // Ignore .text segment
        if TokenKind::INDICATE(IndicateKind::data) == *tokens.kind() {

            // consume EOL
            tokens.consume().unwrap();
            tokens.expect_eol().unwrap();

            // until .text segment
            while tokens.consume().is_some() {
                if TokenKind::INDICATE(IndicateKind::text) == *tokens.kind() {
                    break;
                }

                // TokenKind::LABEL(usize) = data.len() + 1
                if let TokenKind::LABEL(_, _, ref mut index) = &mut tokens.kind() {
                    *index = Some(data.len() + 1);
                } else {
                    break;
                }

                // until EOL
                while tokens.consume().is_some() {
                    if TokenKind::EOL == *tokens.kind() {
                        break;
                    }

                    match tokens.kind() {
                        // Big Endian
                        TokenKind::INDICATE(IndicateKind::word(w)) => {
                            //data.push(((*w>>24) & (std::u32::MAX - 255)) as u8);
                            //data.push(((*w>>16) & (std::u32::MAX - 255)) as u8);
                            //data.push(((*w>> 8) & (std::u32::MAX - 255)) as u8);
                            //data.push(( *w      & (std::u32::MAX - 255)) as u8);
                            data.push((*w>>24) as u8);
                            data.push((*w>>16) as u8);
                            data.push((*w>> 8) as u8);
                            data.push( *w      as u8);
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
                        TokenKind::INDICATE(IndicateKind::align(n)) => {
                            // TODO
                            // Align 2^n
                            //data.len() % 
                        },
                        _ => (),
                    }
                }
            }
        }
    }

    data.push(0);
    tokens.reset();
}

