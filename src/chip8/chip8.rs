use crate::chip8::Memory;

pub struct Chip8 {
    pub memory: Memory
}
impl Chip8 {
    pub fn new() -> Self {
        Self {
            memory: Memory::new()
        }
    }
}