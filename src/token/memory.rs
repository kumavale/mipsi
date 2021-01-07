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
    pub static_data:  Vec<u8>,
    pub dynamic_data: Vec<u8>,
    pub stack:        Vec<u8>,
}

impl Memory {
    // Memory allocation
    // Returns the first assigned address
    pub fn malloc(&mut self, size: i32) -> Result<i32, String> {
        if size <= 0 {
            return Err(format!("invalid memory assign size: {}", size));
        }
        let first = self.dynamic_data.len() as u32 + DYNAMIC_DATA;
        self.dynamic_data.resize(size as usize + self.dynamic_data.len(), 0);
        Ok(first as i32)
    }
}

