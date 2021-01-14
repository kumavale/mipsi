use super::register::Registers;

use wasm_bindgen::prelude::*;

use std::rc::Rc;
use std::cell::{Ref, RefCell};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

// Memory layout
//pub const TEXT_SEGMENT:  u32 = 0x04000000;
pub const STATIC_DATA:   u32 = 0x10000000;
pub const DYNAMIC_DATA:  u32 = 0x20000000;  // tmp
pub const DYNAMIC_DATA_EXIT: u32 = 0x30000000;  // tmp
pub const STACK_SEGMENT: u32 = 0x7fffffff;

#[wasm_bindgen]
#[derive(Clone, Debug, Default)]
pub struct Memory {
    pub registers: Registers,
    pub hi: u32,
    pub lo: u32,
    pub base_address: i32,
    static_data:  Vec<u8>,
    dynamic_data: Vec<u8>,
    stack:        Vec<u8>,
}

impl Memory {
    // Memory allocation
    // Returns the first assigned address
    pub fn malloc(&mut self, size: i32) -> i32 {
        let first = self.dynamic_data.len() as u32 + DYNAMIC_DATA;
        if 0 < size {
            self.dynamic_data.resize(size as usize + self.dynamic_data.len(), 0);
        }
        first as i32
    }

    pub fn clear(&mut self) {
        self.registers = Registers::default();
        self.hi = 0;
        self.lo = 0;
        self.base_address = 0;
        self.static_data.clear();
        self.dynamic_data.clear();
        self.stack.clear();
    }

    #[inline]
    pub fn static_data(&self) -> &[u8] {
        &self.static_data
    }

    #[inline]
    pub fn set_static_data(&mut self, idx: usize, value: u8) {
        self.static_data[idx] = value;
    }

    #[inline]
    pub fn push_static_data(&mut self, value: u8) {
        self.static_data.push(value);
    }

    #[inline]
    pub fn dynamic_data(&self) -> &[u8] {
        &self.dynamic_data
    }

    #[inline]
    pub fn set_dynamic_data(&mut self, idx: usize, value: u8) {
        self.dynamic_data[idx] = value;
    }

    #[inline]
    pub fn stack(&self) -> &[u8] {
        &self.stack
    }

    #[inline]
    pub fn set_stack(&mut self, idx: usize, value: u8) {
        self.stack[idx] = value;
    }

    #[inline]
    pub fn resize_stack(&mut self, new_len: usize) {
        self.stack.resize(new_len, 0);
    }
}

