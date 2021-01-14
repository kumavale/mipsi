use std::io::Write;
use crate::token::register::Registers;

// Display help
// Use only in REPL
#[allow(dead_code)]
pub fn display_help() {
    println!("exit  ... to exit");
    println!("help  ... display this message");
    println!("dispt ... display tokens");
    println!("dispd ... display data");
    println!("disps ... display stack");
    println!("dispr ... display register");
    println!("dispf ... display floating-point register");
}

// Display registers
pub fn display_register(registers: &Registers) {
    println!("\n====[ REGISTER ]================================================");
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

// Display floating-point registers
pub fn display_fp_register(registers: &Registers) {
    println!("\n====[ FLOATING-POINT REGISTER ]=================================");
    for i in 0..8 {
        for j in 0..4 {
            if registers[32+i+j*8] == 0 {
                print!(" ${:<2}:0x{:08x}\t", i+j*8, registers[32+i+j*8]);
            } else {
                print!(" ${:<2}:\x1b[31m0x{:08x}\x1b[m\t", i+j*8, registers[32+i+j*8]);
            }
            std::io::stdout().flush().unwrap();
        }
        println!();
    }
    println!("================================================================");
}

pub fn display_data_per_4byte(data: &[u8]) {
    println!("\n====[ DATA ]====================================================");
    for i in 0..=data.len()/16 {
        print!(" 0x{:08x}:   ", i*16);
        for j in 0..4 {
            let mut int = 0;
            for k in 0..4 {
                if i*16+j*4+k < data.len() {
                    int |= (data[i*16+j*4+k] as i32) << ((4-1-k) * 8);
                }
            }
            if int == 0 {
                print!("  0x{:08x}", int);
            } else {
                print!("  \x1b[31m0x{:08x}\x1b[m", int);
            }
            std::io::stdout().flush().unwrap();
        }
        println!();
    }
    println!("================================================================");
}

pub fn display_stack(stack: &[u8]) {
    println!("\n====[ STACK ]===================================================");
    for i in 0..=stack.len()/16 {
        print!(" 0x{:08x}:   ", i*16);
        for j in 0..4 {
            let mut int = 0;
            for k in 0..4 {
                if i*16+j*4+k < stack.len() {
                    int |= (stack[i*16+j*4+k] as i32) << ((4-1-k) * 8);
                }
            }
            if int == 0 {
                print!("  0x{:08x}", int);
            } else {
                print!("  \x1b[31m0x{:08x}\x1b[m", int);
            }
            std::io::stdout().flush().unwrap();
        }
        println!();
    }
    println!("================================================================");
}

