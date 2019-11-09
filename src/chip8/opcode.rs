#[derive(Debug)]
pub enum Opcode {
    /// __0*nnn* - SYS *addr*__
    ///
    /// Call RCA1802 program at address *nnn*.
    _0nnn { n: u16 },
    /// __00E0 - CLS__
    ///
    /// Clear the screen.
    _00E0,
    /// __00EE - RET__
    ///
    /// Return from a subroutine.
    _00EE,
    /// __1*nnn* - JP *addr*__
    ///
    /// Jump to location at *nnn*.
    _1nnn { n: u16 },
    /// __2*nnn* - CALL *addr*__
    ///
    /// Call subroutine at *nnn*.
    _2nnn { n: u16 },
    /// __3*xkk* - SE V*x*, *byte*__
    ///
    /// Skip the next instruction if V*x* == *kk*.
    _3xkk { x: u8, k: u8 },
    /// __4*xkk* - SNE V*x*, *byte*__
    ///
    /// Skip the next instruction if V*x* != *kk*.
    _4xkk { x: u8, k: u8 },
    /// __5*xy*0 - SE V*x*, V*y*__
    ///
    /// Skip the next instruction if V*x* == V*y*.
    _5xy0 { x: u8, y: u8 },
    /// __6*xkk* - LD V*x*, *byte*__
    ///
    /// Set V*x* = *kk*.
    _6xkk { x: u8, k: u8 },
    /// __7*xkk* - ADD V*x*, *byte*__
    ///
    /// Set V*x* = V*x* + *kk*.
    _7xkk { x: u8, k: u8 },
    /// __8*xy*0 - LD V*x*, V*y*__
    ///
    /// Set V*x* = V*y*.
    _8xy0 { x: u8, y: u8 },
    /// __8*xy*1 - OR V*x*, V*y*__
    ///
    /// Set V*x* = V*x* OR V*y*.
    _8xy1 { x: u8, y: u8 },
    /// __8*xy*2 - AND V*x*, V*y*__
    ///
    /// Set V*x* = V*x* AND V*y*.
    _8xy2 { x: u8, y: u8 },
    /// __8*xy*3 - XOR V*x*, V*y*__
    ///
    /// Set V*x* = V*x* XOR V*y*.
    _8xy3 { x: u8, y: u8 },
    /// __8*xy*4 - ADD V*x*, V*y*__
    ///
    /// Set V*x* = V*x* + V*y*, set VF = *carry*.
    _8xy4 { x: u8, y: u8 },
    /// __8*xy*5 - SUB V*x*, V*y*__
    ///
    /// Set V*x* = V*x* - V*y*, set VF = NOT *borrow*.
    _8xy5 { x: u8, y: u8 },
    /// __8*xy*6 - SHR V*x*{, V*y*}__
    ///
    /// Set V*x* = V*x* SHR 1.
    _8xy6 { x: u8, y: u8 },
    /// __8*xy*7 - SUBN V*x*, V*y*__
    ///
    /// Set V*x* = V*y* - V*x*, set VF = NOT *borrow*.
    _8xy7 { x: u8, y: u8 },
    /// __8*xy*E - SHL V*x*{, V*y*}__
    ///
    /// Set V*x* = V*x* SHL 1.
    _8xyE { x: u8, y: u8 },
    /// __9*xy*0 - SNE V*x*, V*y*__
    ///
    /// Skip next instruction if V*x* != V*y*.
    _9xy0 { x: u8, y: u8 },
    /// __A*nnn* - LD I, *addr*__
    ///
    /// Set I = *nnn*.
    _Annn { n: u16 },
    /// __B*nnn* - JP V0,*addr*__
    ///
    /// Jump to location *nnn* + V0.
    _Bnnn { n: u16 },
    /// __C*xkk* - RND V*x*, *byte*__
    ///
    /// Set V*x* = *random byte* AND *kk*.
    _Cxkk { x: u8, k: u8 },
    /// __D*xyn* - DRW V*x*, V*y*, *nibble*__
    ///
    /// Display *n*-byte sprite starting at memory location I at (V*x*, V*y*), set VF = *collision*.
    _Dxyn { x: u8, y: u8, n: u8 },
    /// __E*x*9E - SKP V*x*__
    ///
    /// Skip the next instruction if key with the value of V*x* is pressed.
    _Ex9E { x: u8 },
    /// __E*x*A1 - SKNP V*x*__
    ///
    /// Skip the next instruction if key with the value of V*x* is not pressed.
    _ExA1 { x: u8 },
    /// __F*x*07 - LD V*x*, DT__
    ///
    /// Set V*x* = *delay timer*.
    _Fx07 { x: u8 },
    /// __F*x*0A - LD V*x*, K__
    ///
    /// Wait for a key press, store the value of the key in V*x*.
    _Fx0A { x: u8 },
    /// __F*x*15 - LD DT, V*x*__
    ///
    /// Set *delay timer* = V*x*.
    _Fx15 { x: u8 },
    /// __F*x*18 - LD ST, V*x*__
    ///
    /// Set *sound timer* = V*x*.
    _Fx18 { x: u8 },
    /// __F*x*1E - ADD I, V*x*__
    ///
    /// Set I = I = V*x*.
    _Fx1E { x: u8 },
    /// __F*x*29 - LD F, V*x*__
    ///
    /// Set I = location for sprite for digit V*x*.
    _Fx29 { x: u8 },
    /// __F*x*33 - LD B, V*x*__
    ///
    /// Store BCD representation of V*x* in memory locations I, I+1, and I+2.
    _Fx33 { x: u8 },
    /// __F*x*55 - LD [I], V*x*__
    ///
    /// Store registers V0 through V*x* in memory starting at location I.
    _Fx55 { x: u8 },
    /// __F*x*65 - LD V*x*, [I]__
    ///
    /// Read registers V0 through V*x* from memory starting at location I.
    _Fx65 { x: u8 },
    /// Invalid opcode.
    Invalid { code: u16 }
}
impl Opcode {
    pub fn from(instruction: u16) -> Opcode {
        match instruction {
            0x00E0 => Opcode::_00E0,
            0x00EE => Opcode::_00EE,
            _ => match instruction & 0xF000 {
                0x0000 => Opcode::_0nnn { n: instruction & 0x0FFF },
                0x1000 => Opcode::_1nnn { n: instruction & 0x0FFF },
                0x2000 => Opcode::_2nnn { n: instruction & 0x0FFF },
                0x3000 => Opcode::_3xkk { x: ((instruction & 0x0F00) >> 8) as u8, k: (instruction & 0x00FF) as u8 },
                0x4000 => Opcode::_4xkk { x: ((instruction & 0x0F00) >> 8) as u8, k: (instruction & 0x00FF) as u8 },
                0x5000 => match instruction & 0x000F {
                    0x0000 => Opcode::_5xy0 { x: ((instruction & 0x0F00) >> 8) as u8, y: ((instruction & 0x00F0) >> 4) as u8 },
                    _ => Opcode::Invalid { code: instruction }
                },
                0x6000 => Opcode::_6xkk { x: ((instruction & 0x0F00) >> 8) as u8, k: (instruction & 0x00FF) as u8 },
                0x7000 => Opcode::_7xkk { x: ((instruction & 0x0F00) >> 8) as u8, k: (instruction & 0x00FF) as u8 },
                0x8000 => match instruction & 0x000F {
                    0x0000 => Opcode::_8xy0 { x: ((instruction & 0x0F00) >> 8) as u8, y: ((instruction & 0x00F0) >> 4) as u8 },
                    0x0001 => Opcode::_8xy1 { x: ((instruction & 0x0F00) >> 8) as u8, y: ((instruction & 0x00F0) >> 4) as u8 },
                    0x0002 => Opcode::_8xy2 { x: ((instruction & 0x0F00) >> 8) as u8, y: ((instruction & 0x00F0) >> 4) as u8 },
                    0x0003 => Opcode::_8xy3 { x: ((instruction & 0x0F00) >> 8) as u8, y: ((instruction & 0x00F0) >> 4) as u8 },
                    0x0004 => Opcode::_8xy4 { x: ((instruction & 0x0F00) >> 8) as u8, y: ((instruction & 0x00F0) >> 4) as u8 },
                    0x0005 => Opcode::_8xy5 { x: ((instruction & 0x0F00) >> 8) as u8, y: ((instruction & 0x00F0) >> 4) as u8 },
                    0x0006 => Opcode::_8xy6 { x: ((instruction & 0x0F00) >> 8) as u8, y: ((instruction & 0x00F0) >> 4) as u8 },
                    0x0007 => Opcode::_8xy7 { x: ((instruction & 0x0F00) >> 8) as u8, y: ((instruction & 0x00F0) >> 4) as u8 },
                    0x000E => Opcode::_8xyE { x: ((instruction & 0x0F00) >> 8) as u8, y: ((instruction & 0x00F0) >> 4) as u8 },
                    _ => Opcode::Invalid { code: instruction }
                },
                0x9000 => match instruction & 0x000F {
                    0x0000 => Opcode::_9xy0 { x: ((instruction & 0x0F00) >> 8) as u8, y: ((instruction & 0x00F0) >> 4) as u8 },
                    _ => Opcode::Invalid { code: instruction }
                },
                0xA000 => Opcode::_Annn { n: instruction & 0x0FFF },
                0xB000 => Opcode::_Bnnn { n: instruction & 0x0FFF },
                0xC000 => Opcode::_Cxkk { x: ((instruction & 0x0F00) >> 8) as u8, k: (instruction & 0x00FF) as u8 },
                0xD000 => Opcode::_Dxyn { x: ((instruction & 0x0F00) >> 8) as u8, y: ((instruction & 0x00F0) >> 4) as u8, n: (instruction & 0x000F) as u8 },
                0xE000 => match instruction & 0x00FF {
                    0x009E => Opcode::_Ex9E { x: ((instruction & 0x0F00) >> 8) as u8 },
                    0x00A1 => Opcode::_ExA1 { x: ((instruction & 0x0F00) >> 8) as u8 },
                    _ => Opcode::Invalid { code: instruction }
                },
                0xF000 => match instruction & 0x00FF {
                    0x0007 => Opcode::_Fx07 { x: ((instruction & 0x0F00) >> 8) as u8 },
                    0x000A => Opcode::_Fx0A { x: ((instruction & 0x0F00) >> 8) as u8 },
                    0x0015 => Opcode::_Fx15 { x: ((instruction & 0x0F00) >> 8) as u8 },
                    0x0018 => Opcode::_Fx18 { x: ((instruction & 0x0F00) >> 8) as u8 },
                    0x001E => Opcode::_Fx1E { x: ((instruction & 0x0F00) >> 8) as u8 },
                    0x0029 => Opcode::_Fx29 { x: ((instruction & 0x0F00) >> 8) as u8 },
                    0x0033 => Opcode::_Fx33 { x: ((instruction & 0x0F00) >> 8) as u8 },
                    0x0055 => Opcode::_Fx55 { x: ((instruction & 0x0F00) >> 8) as u8 },
                    0x0065 => Opcode::_Fx65 { x: ((instruction & 0x0F00) >> 8) as u8 },
                    _ => Opcode::Invalid { code: instruction }
                },
                _ => Opcode::Invalid { code: instruction }
            }
        }
    }
}