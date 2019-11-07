use crate::chip8::{Processor, Memory};

pub struct Chip8 {
    pub processor: Processor,
    pub memory: Memory
}
impl Chip8 {
    pub fn new() -> Self {
        Self {
            processor: Processor::new(),
            memory: Memory::new()
        }
    }
}