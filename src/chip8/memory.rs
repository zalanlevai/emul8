use crate::chip8::memory::MemoryError::*;
use std::convert::TryFrom;
use std::array::TryFromSliceError;

pub struct Memory {
    /// The raw data that makes up the system memory.
    ///
    /// Memory map:<br>
    /// 0x000 - 0x1FF: Chip-8 Interpreter<br>
    /// 0x050 - 0x0A0: Used for the built-int 4x5 pixel font set (0-F).<br>
    /// 0x200 - 0xFFF: Program ROM and Work RAM
    data: [u8; 4096]
}
impl Memory {
    pub fn new() -> Self {
        Self {
            data: [0; 4096]
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }
    pub fn read_16(&self, addr: u16) -> Result<u16, MemoryError> {
        Ok(u16::from_be_bytes(<[u8;2]>::try_from(self.read_many(addr, 2))?))
    }
    pub fn read_many(&self, addr: u16, len: u16) -> &[u8] {
        self.read_range(addr, addr+len)
    }
    pub fn read_range(&self, from: u16, to: u16) -> &[u8] {
        &self.data[(from as usize)..(to as usize)]
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.data[addr as usize] = val;
    }
    pub fn copy(&mut self, addr: u16, val: &[u8]) {
        for (i, mem) in val.iter().enumerate() {
            self.data[addr as usize + i] = *mem;
        }
    }
}

pub enum MemoryError {
    MemoryAccessError
}
impl From<TryFromSliceError> for MemoryError {
    fn from(_: TryFromSliceError) -> Self {
        MemoryAccessError
    }
}