use super::register::Registers;

// Memory layout
//pub const TEXT_SEGMENT:  u32 = 0x04000000;
pub const STATIC_DATA:   u32 = 0x10000000;
pub const DYNAMIC_DATA:  u32 = 0x20000000;  // tmp
pub const DYNAMIC_DATA_EXIT: u32 = 0x30000000;  // tmp
pub const STACK_SEGMENT: u32 = 0x7fffffff;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Memory {
    pub registers: Registers,
    pub hi: u32,
    pub lo: u32,
    pub base_address: i32,
    pub static_data:  Vec<u8>,
    pub dynamic_data: Vec<u8>,
    pub stack:        Vec<u8>,
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
}

