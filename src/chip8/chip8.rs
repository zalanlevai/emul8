use crate::chip8::{Processor, Memory, Bios};

pub struct Chip8 {
    pub processor: Processor,
    pub memory: Memory,
    pub bios: Bios
}
impl Chip8 {
    pub fn new() -> Self {
        Self {
            processor: Processor::new(),
            memory: Memory::new(),
            bios: Bios::new()
        }
    }

    pub fn init(&mut self) {
        self.bios.load(&mut self.memory);
    }
}