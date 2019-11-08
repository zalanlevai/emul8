use crate::chip8::{Chip8, Opcode};
use crate::chip8::opcode::Opcode::*;
use crate::chip8::memory::MemoryError;
use crate::chip8::memory::MemoryError::*;
use crate::chip8::processor::ProcessorError::*;

pub struct Processor {
    operation: Opcode,
    registers: Registers
}
impl Processor {
    pub fn new() -> Self {
        Self {
            operation: Opcode::INVALID { code: 0x0000 },
            registers: Registers::new()
        }
    }

    fn execute_from(system: &mut Chip8, instruction: u16) {
        Processor::execute(system, Opcode::from(instruction));
    }
    fn execute(system: &mut Chip8, operation: Opcode) {
        system.processor.operation = operation;

        println!("Stepping into opcode: {:?}", system.processor.operation);

        match system.processor.operation {
            _0nnn { n } => { panic!(); },
            _00E0 => { },
            _00EE => { },
            _1nnn { n } => { },
            _2nnn { n } => { },
            _3xkk { x, k } => { },
            _4xkk { x, k } => { },
            _5xy0 { x, y } => { },
            _6xkk { x, k } => { },
            _7xkk { x, k } => { },
            _8xy0 { x, y } => { },
            _8xy1 { x, y } => { },
            _8xy2 { x, y } => { },
            _8xy3 { x, y } => { },
            _8xy4 { x, y } => { },
            _8xy5 { x, y } => { },
            _8xy6 { x, y } => { },
            _8xy7 { x, y } => { },
            _8xyE { x, y } => { },
            _9xy0 { x, y } => { },
            _Annn { n } => { },
            _Bnnn { n } => { },
            _Cxkk { x, k } => { },
            _Dxyn { x, y, n } => { },
            _Ex9E { x } => { },
            _ExA1 { x } => { },
            _Fx07 { x } => { },
            _Fx0A { x } => { },
            _Fx15 { x } => { },
            _Fx18 { x } => { },
            _Fx1E { x } => { },
            _Fx29 { x } => { },
            _Fx33 { x } => { },
            _Fx55 { x } => { },
            _Fx65 { x } => { },
            INVALID { code } => { }
        }

        system.processor.registers.pc += 2;
    }
}
impl Processor {
    pub fn cycle(system: &mut Chip8) -> Result<(), ProcessorError> {
        println!("{:#X}", system.memory.read_16(system.processor.registers.pc)?);

        Processor::execute_from(system, system.memory.read_16(system.processor.registers.pc)?);

        Ok(())
    }
}

pub struct Registers {
    /// Program Counter
    pc: u16,
    /// Stack Pointer
    sp: u8,

    /// Stack; used to store the address to return to when finished with a subroutine.
    stack: [u16; 16],

    /// General purpose registers (V0, V1, ..., VF)
    v: [u8; 16],
    /// 16-bit general purpose register
    i: u16,
    /// Delay Timer Register
    delay_timer: u8,
    /// Sound Timer Register
    sound_timer: u8
}
impl Registers {
    pub fn new() -> Self {
        Self {
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
            v: [0; 16],
            i: 0,
            delay_timer: 0,
            sound_timer: 0
        }
    }
}

pub enum ProcessorError {
    InvalidOpcodeError
}
impl From<MemoryError> for ProcessorError {
    fn from(err: MemoryError) -> Self {
        match err {
            // TODO: Better error handling
            MemoryAccessError => InvalidOpcodeError
        }
    }
}