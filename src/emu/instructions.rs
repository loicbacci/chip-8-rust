//! Code used to decode the instructions

#[allow(clippy::upper_case_acronyms)]

/// Enum used to represent CPU instructions
#[derive(Debug)]
pub enum Instr {
    /// Clear screen
    CLS,
    /// Return from subroutine
    RET,
    /// Jump
    JP(u16),
    /// Go to a subroutine
    CALL(u16),
    /// Skip if equal
    SE(u8, u8),
    /// Skip if not equal
    SNE(u8, u8),
    /// Skip if equal (using two registers)
    SEV(u8, u8),
    /// Set register
    LD(u8, u8),
    /// Addition
    ADD(u8, u8),
    /// Set register from another register
    LDV(u8, u8),
    /// Or
    OR(u8, u8),
    /// And
    AND(u8, u8),
    /// Xor
    XOR(u8, u8),
    /// Add two registers
    ADDV(u8, u8),
    /// Subtract
    SUB(u8, u8),
    /// Shift right
    SHR(u8, u8),
    /// Subtract vy from vx
    SUBN(u8, u8),
    /// Shift left
    SHL(u8, u8),
    /// Skip not equal (using registers)
    SNEV(u8, u8),
    /// Set register I
    LDI(u16),
    /// Jump using vx or v0 (COSMAC)
    JPV(u8, u16),
    /// Generate random number
    RND(u8, u8),
    /// Draw
    DRW(u8, u8, u8),
    /// Skip if key pressed
    SKP(u8),
    /// Skip if key not pressed
    SKNP(u8),
    /// Load x into delay timer
    LDXT(u8),
    /// Get key
    LDK(u8),
    /// Load delay timer into x
    LDTX(u8),
    /// Load sound timer into x
    LDS(u8),
    /// Add to the register I
    ADDI(u8),
    /// Loads into I the address for a given character
    LDF(u8),
    /// Get BCD representation of argument
    LDB(u8),
    /// Store into memory
    LDIX(u8),
    /// Load from memory
    LDXI(u8),

    /// Every other instruction
    NIL,
}

impl Instr {
    /// Decode an instruction given in bits
    pub fn decode(fe: u16) -> Instr {
        use crate::emu::instructions::Instr::*;

        let x = ((fe & 0x0F00) >> 8) as u8;
        let y = ((fe & 0x00F0) >> 4) as u8;
        let n = (fe & 0x000F) as u8;
        let nn = (fe & 0x00FF) as u8;
        let nnn = fe & 0x0FFF;

        match fe {
            0x00E0 => return CLS,
            0x00EE => return RET,
            _ => ()
        }

        match (fe & 0xF000) >> 12 {
            0x1 => return JP(nnn),
            0x2 => return CALL(nnn),
            0x3 => return SE(x, nn),
            0x4 => return SNE(x, nn),
            0x5 => return SEV(x, y),
            0x6 => return LD(x, nn),
            0x7 => return ADD(x, nn),

            0x8 => match n {
                0x0 => return LDV(x, y),
                0x1 => return OR(x, y),
                0x2 => return AND(x, y),
                0x3 => return XOR(x, y),
                0x4 => return ADDV(x, y),
                0x5 => return SUB(x, y),
                0x6 => return SHR(x, y),
                0x7 => return SUBN(x, y),
                0xE => return SHL(x, y),
                _ => ()
            },

            0x9 => return SNEV(x, y),
            0xA => return LDI(nnn),
            0xB => return JPV(x, nnn),
            0xC => return RND(x, nn),
            0xD => return DRW(x, y, n),
            
            0xE => match nn {
                0x9E => return SKP(x),
                0xA1 => return SKNP(x),
                _ => (),
            }

            0xF => match nn {
                0x07 => return LDXT(x),
                0x0A => return LDK(x),
                0x15 => return LDTX(x),
                0x18 => return LDS(x),
                0x1E => return ADDI(x),
                0x29 => return LDF(x),
                0x33 => return LDB(x),
                0x55 => return LDIX(x),
                0x65 => return LDXI(x),
                _ => (),
            }
            _ => ()
        }

        Instr::NIL
    }
}
