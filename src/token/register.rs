use std::ops::{Index, IndexMut};
use super::memory::*;

#[derive(Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum RegisterKind {
    zero,                            //     0: Hard-wired to 0
    at,                              //     1: Reserved for pseudo-instructions
    v0, v1,                          //   2-3: Return values from functions
    a0, a1, a2, a3,                  //   4-7: Arguments to functions - not preserved by subprograms
    t0, t1, t2, t3, t4, t5, t6, t7,  //  8-15: Temporary data, not preserved by subprograms
    s0, s1, s2, s3, s4, s5, s6, s7,  // 16-23: Saved registers, preserved by subprograms
    t8, t9,                          // 24-25: More temporary registers, not preserved by subprograms
    k0, k1,                          // 26-27: Reserved for kernel. Do not use.
    gp,                              //    28: Global Area Pointer (base of global data segment)
    sp,                              //    29: Stack Pointer
    fp,                              //    30: Frame Pointer
    ra,                              //    31: Return Address

    // Floating-Point registers
    f0, f2,                                    // Hold results of floating-point type function
    f1, f3, f4, f5, f6, f7, f8, f9, f10, f11,  // Temporary registers
    f12, f13, f14, f15, f16, f17, f18, f19,    // Pass single or double precision actual arguments
    f20, f21, f22, f23,                        // Temporary registers
    f24, f25, f26, f27, f28, f29, f30, f31,    // Saved registers
    fcsr,                                      // FPU control and status register
}

#[derive(Clone, Debug, PartialEq)]
pub struct Registers {
    regs: [i32; 32+32+1],
}

impl Default for Registers {
    fn default() -> Self {
        let mut regs = [0i32; 32+32+1];
        regs[RegisterKind::sp as usize] = STACK_SEGMENT as i32;  // init stack pointer
        Self {
            regs,
        }
    }
}

impl Index<usize> for Registers {
    type Output = i32;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &0,
            _ => &self.regs[idx],
        }
    }
}

impl IndexMut<usize> for Registers {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.regs[idx]
    }
}

// [ Consider... ]
//#[derive(Clone, Copy)]
//pub union Register {
//    pub i: i32,
//    pub u: u32,
//}
//
//impl Register {
//    pub fn new() -> Self {
//        Register { i: 0 }
//    }
//}

