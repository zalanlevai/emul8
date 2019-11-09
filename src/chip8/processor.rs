use crate::chip8::{Chip8, Opcode};
use crate::chip8::memory::MemoryError;
use crate::chip8::memory::MemoryError::*;
use crate::chip8::processor::ProcessorError::*;
use std::convert::TryFrom;
use std::array::TryFromSliceError;
use std::fmt::Write;

pub struct Processor {
    operation: Opcode,
    registers: Registers
}
impl Processor {
    pub fn new() -> Self {
        Self {
            operation: Opcode::Invalid { code: 0x0000 },
            registers: Registers::new()
        }
    }

    fn execute_from(system: &mut Chip8, instruction: u16) -> Result<(), ProcessorError> {
        Processor::execute(system, Opcode::from(instruction))
    }
    fn execute(system: &mut Chip8, operation: Opcode) -> Result<(), ProcessorError> {
        system.processor.operation = operation;

        println!("Stepping into opcode: {:?}", system.processor.operation);

        let mut dont_step = false;

        match system.processor.operation {
            Opcode::_0nnn { n } => { panic!(); },
            Opcode::_00E0 => { },
            Opcode::_00EE => {
                system.processor.registers.pop_stack();
            },
            Opcode::_1nnn { n } => {
                system.processor.registers.pc = n;
                dont_step = true;
            },
            Opcode::_2nnn { n } => {
                system.processor.registers.push_pc_stack();
                system.processor.registers.pc = n;
                dont_step = true;
            },
            Opcode::_3xkk { x, k } => {
                if system.processor.registers.read_v(x) == k {
                    system.processor.registers.pc += 2;
                }
            },
            Opcode::_4xkk { x, k } => {
                if system.processor.registers.read_v(x) != k {
                    system.processor.registers.pc += 2;
                }
            },
            Opcode::_5xy0 { x, y } => {
                if system.processor.registers.read_v(x) == system.processor.registers.read_v(y) {
                    system.processor.registers.pc += 2;
                }
            },
            Opcode::_6xkk { x, k } => {
                system.processor.registers.write_v(x, k);
            },
            Opcode::_7xkk { x, k } => {
                let vx = system.processor.registers.read_v(x);
                let (val, _) = vx.overflowing_add(k);
                system.processor.registers.write_v(x, val);
            },
            Opcode::_8xy0 { x, y } => {
                system.processor.registers.write_v(x, system.processor.registers.read_v(y));
            },
            Opcode::_8xy1 { x, y } => {
                let (vx, vy) = (system.processor.registers.read_v(x), system.processor.registers.read_v(y));
                system.processor.registers.write_v(x, vx | vy);
            },
            Opcode::_8xy2 { x, y } => {
                let (vx, vy) = (system.processor.registers.read_v(x), system.processor.registers.read_v(y));
                system.processor.registers.write_v(x, vx & vy);
            },
            Opcode::_8xy3 { x, y } => {
                let (vx, vy) = (system.processor.registers.read_v(x), system.processor.registers.read_v(y));
                system.processor.registers.write_v(x, vx ^ vy);
            },
            Opcode::_8xy4 { x, y } => {
                let (vx, vy) = (system.processor.registers.read_v(x), system.processor.registers.read_v(y));
                let (val, carry) = vx.overflowing_add(vy);
                system.processor.registers.write_v(x, val);
                system.processor.registers.write_v(0xF, carry as u8);
            },
            Opcode::_8xy5 { x, y } => {
                let (vx, vy) = (system.processor.registers.read_v(x), system.processor.registers.read_v(y));
                let (val, borrow) = vx.overflowing_sub(vy);
                system.processor.registers.write_v(x, val);
                system.processor.registers.write_v(0xF, (!borrow) as u8);
            },
            Opcode::_8xy6 { x, y: _ } => {
                let vx = system.processor.registers.read_v(x);
                system.processor.registers.write_v(0xF, vx & 0x1);
                system.processor.registers.write_v(x, vx >> 1);
            },
            Opcode::_8xy7 { x, y } => {
                let (vx, vy) = (system.processor.registers.read_v(x), system.processor.registers.read_v(y));
                let (val, borrow) = vy.overflowing_sub(vx);
                system.processor.registers.write_v(x, val);
                system.processor.registers.write_v(0xF, (!borrow) as u8);
            },
            Opcode::_8xyE { x, y } => {
                let vx = system.processor.registers.read_v(x);
                system.processor.registers.write_v(0xF, vx >> 0x7);
                system.processor.registers.write_v(x, vx << 1);
            },
            Opcode::_9xy0 { x, y } => {
                if system.processor.registers.read_v(x) != system.processor.registers.read_v(y) {
                    system.processor.registers.pc += 2;
                }
            },
            Opcode::_Annn { n } => {
                system.processor.registers.i = n;
            },
            Opcode::_Bnnn { n } => {
                system.processor.registers.pc = n + system.processor.registers.read_v(0x0) as u16;
                dont_step = true;
            },
            Opcode::_Cxkk { x, k } => { },
            Opcode::_Dxyn { x, y, n } => { },
            Opcode::_Ex9E { x } => { },
            Opcode::_ExA1 { x } => { },
            Opcode::_Fx07 { x } => {
                system.processor.registers.write_v(x, system.processor.registers.delay_timer);
            },
            Opcode::_Fx0A { x } => { },
            Opcode::_Fx15 { x } => {
                system.processor.registers.delay_timer = system.processor.registers.read_v(x);
            },
            Opcode::_Fx18 { x } => {
                system.processor.registers.sound_timer = system.processor.registers.read_v(x);
            },
            Opcode::_Fx1E { x } => {
                system.processor.registers.i += system.processor.registers.read_v(x) as u16;
            },
            Opcode::_Fx29 { x } => {
                system.processor.registers.i = 0x50 + (system.processor.registers.read_v(x) as u16) * 0x5;
            },
            Opcode::_Fx33 { x } => { },
            Opcode::_Fx55 { x } => {
                system.memory.copy(system.processor.registers.i, &system.processor.registers.v);
            },
            Opcode::_Fx65 { x } => {
                system.processor.registers.v = <[u8; 16]>::try_from(system.memory.read_many(system.processor.registers.i, 0xF))?;
            },
            Opcode::Invalid { code } => { return Err(ProcessorError::InvalidOpcodeError); }
        }

        if !dont_step {
            system.processor.registers.pc += 2;
        }

        Ok(())
    }
}
impl Processor {
    pub fn cycle(system: &mut Chip8) -> Result<(), ProcessorError> {
        println!("{:#X}", system.memory.read_16(system.processor.registers.pc)?);

        Processor::execute_from(system, system.memory.read_16(system.processor.registers.pc)?)
    }

    pub fn halt(system: &Chip8) {
        println!("CPU halted!\n");
        println!("{}", system.processor.registers.dump());
        println!("{}", system.memory.dump());
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

    pub fn read_v(&self, i: u8) -> u8 {
        self.v[i as usize]
    }
    pub fn write_v(&mut self, i: u8, val: u8) {
        self.v[i as usize] = val;
    }

    pub fn peek_stack(&self) -> u16 {
        self.stack[self.sp as usize]
    }
    pub fn pop_stack(&mut self) -> u16 {
        self.pc = self.peek_stack();
        self.sp -= 1;
        self.pc
    }
    pub fn push_pc_stack(&mut self) {
        self.push_stack(self.pc);
    }
    pub fn push_stack(&mut self, addr: u16) {
        self.sp += 1;
        self.stack[self.sp as usize] = addr;
    }

    pub fn dump(&self) -> String {
        let mut writer = String::new();

        // Program Counter
        writeln!(writer, "{:<4}{:#X}", "PC", self.pc);

        // Stack info
        writeln!(writer, "{:<4}{:#X}", "SP",  self.sp);
        writeln!(writer, "stack:");
        for (i, addr) in self.stack.iter().rev().enumerate() {
            writeln!(writer, "  {} {:#06X}",
                     if self.sp == (0xF - i as u8) { ">" } else { " " }, addr);
        }
        writeln!(writer, "");

        // Registers
        writeln!(writer, "registers:");
        for (i, row) in self.v.chunks(2).enumerate() {
            writeln!(writer, "{:<4}{:<#8X}{:<4}{:#X}",
                     format!("V{:X}", i*2), row[0],
                     format!("V{:X}", i*2+1), row[1]);
        }
        writeln!(writer, "{:<4}{:#X}", "I",  self.i);
        writeln!(writer, "{:<4}{:#X}", "DT", self.delay_timer);
        writeln!(writer, "{:<4}{:#X}", "ST", self.sound_timer);

        writer
    }
}

pub enum ProcessorError {
    InvalidOpcodeError
}
// TODO: Better error handling
impl From<MemoryError> for ProcessorError {
    fn from(err: MemoryError) -> Self {
        match err {
            MemoryAccessError => InvalidOpcodeError
        }
    }
}
impl From<TryFromSliceError> for ProcessorError {
    fn from(_: TryFromSliceError) -> Self {
        InvalidOpcodeError
    }
}