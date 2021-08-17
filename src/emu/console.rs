//! Contains the code pertaining to the console

use crate::emu::instructions::*;
use crate::emu::{box_border, font};
use std::fs;
use rand::rngs::ThreadRng;
use rand::Rng;

/// Width of the buffer
pub const BUFF_WIDTH: usize = 64;
/// Height of the buffer
pub const BUFF_HEIGHT: usize = 32;
/// Total size of the buffer
pub const BUFF_SIZE: usize = BUFF_HEIGHT * BUFF_WIDTH;
/// Total size of the memory
const MEM_SIZE: usize = 4096;

/// Struct containing the variables of a chip-8 console
pub struct Console {
    /// Array of the bytes of the memory
    memory: [u8; MEM_SIZE],
    /// Buffer (display bits) of the console
    buffer: [bool; BUFF_SIZE],

    /// Program counter
    pc: u16,
    /// I register
    i: u16,
    /// Stack
    stack: Vec<u16>,
    /// Delay timer
    delay_timer: u8,
    /// Sound timer
    sound_timer: u8,

    /// Registers
    v: [u8; 16],

    /// Keys of the console
    /// True if they are down, false if they are up
    keys: [bool; 16],

    /// Random number generator
    rng: ThreadRng,
}

impl Console {
    /// Creates a new console
    pub fn new() -> Self {
        let mut cons = Console {
            memory: [0; MEM_SIZE],
            buffer: [false; BUFF_SIZE],
            pc: 0x200,
            i: 0,
            stack: vec![],
            delay_timer: 0,
            sound_timer: 0,
            v: [0; 16],
            keys: [false; 16],
            rng: rand::thread_rng(),
        };

        cons.put_font();

        cons
    }

    /// Gets the value of the bit in the buffer at position (x, y)
    pub fn get_bit(&self, x: usize, y: usize) -> bool {
        self.buffer[x + y * BUFF_WIDTH]
    }

    /// Loads a ROM into the console
    pub fn load_rom(&mut self, filename: &str) {
        let cont = fs::read(filename).expect("Error reading file");
        for (i, bt) in cont.iter().enumerate() {
            self.memory[i + 0x200] = *bt;
        }
    }

    /// Puts the font into the memory
    fn put_font(&mut self) {
        let mut mem_index = font::FONT_ADDR as usize;

        for digit in font::FONT {
            for bt in digit {
                self.memory[mem_index] = bt;
                mem_index += 1;
            }
        }
    }

    /// XORs the bit at position (x,y) in the buffer with the given argument
    fn xor_bit(&mut self, x: usize, y: usize, bit: bool) {
        let bval: bool = self.buffer[x + y * BUFF_WIDTH];
        if bit && !bval || !bit && bval {
            self.buffer[x + y * BUFF_WIDTH] = true;
        } else {
            self.buffer[x + y * BUFF_WIDTH] = false;
        }
    }

    /// Clears the buffer
    fn clear_buffer(&mut self) {
        for i in 0..BUFF_SIZE {
            self.buffer[i] = false;
        }
    }

    /// Sets a given key
    pub fn set_key(&mut self, index: usize, down: bool) {
        self.keys[index] = down;
    }

    /// Goes through a cycle (fetch, decode, execute)
    /// Prints the instruction if debug is true
    /// Draws the buffer into stdout if draw_term is true
    /// Returns true if we need to draw
    pub fn cycle(&mut self, debug: bool, draw_term: bool) -> (bool, bool) {
        let instr_enc = self.fetch();
        let instr_dec = Instr::decode(instr_enc);

        if debug {
            println!("{:04x} - {:?}", instr_enc, instr_dec);
        }
        
        let draw = self.execute(instr_dec);
        if draw_term && draw {
            self.print_buffer();
        }

        // Decrement counters
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }

        (draw, self.sound_timer > 0)
    }
}

// INSTRUCTIONS

impl Console {
    /// Fetches the next instruction (not decoded)
    fn fetch(&mut self) -> u16 {
        let pcu = self.pc as usize;
        let b0 = self.memory[pcu] as u16;
        let b1 = self.memory[pcu + 1] as u16;
        self.pc += 2;

        (b0 << 8) | b1
    }

    /// Executes the given instruction
    fn execute(&mut self, instr: Instr) -> bool {
        let mut draw = false;

        match instr {
            Instr::CLS => self.clear_buffer(),

            Instr::RET => (self.pc = self.stack.pop().expect("Empty stack")),

            Instr::JP(nnn) => self.pc = nnn,

            Instr::CALL(nnn) => {
                self.stack.push(self.pc);
                self.pc = nnn;
            }

            Instr::SE(x, nn) => {
                if self.v[x as usize] == nn {
                    self.pc += 2;
                }
            }

            Instr::SNE(x, nn) => {
                if self.v[x as usize] != nn {
                    self.pc += 2;
                }
            }

            Instr::SEV(x, y) => {
                if self.v[x as usize] == self.v[y as usize] {
                    self.pc += 2;
                }
            }

            Instr::LD(x, nn) => self.v[x as usize] = nn,

            Instr::ADD(x, nn) => {
                self.v[x as usize] = self.v[x as usize].wrapping_add(nn);
            },

            Instr::LDV(x, y) => self.v[x as usize] = self.v[y as usize],

            Instr::OR(x, y) => self.v[x as usize] |= self.v[y as usize],

            Instr::AND(x, y) => self.v[x as usize] &= self.v[y as usize],

            Instr::XOR(x, y) => self.v[x as usize] ^= self.v[y as usize],

            Instr::ADDV(x, y) => {
                let added = self.v[x as usize] as u16 + self.v[y as usize] as u16;
                self.v[x as usize] = added as u8;

                self.v[0xF] = if added > 255 { 1 } else { 0 };
            }

            Instr::SUB(x, y) => {
                self.v[0xF] = if self.v[x as usize] > self.v[y as usize] {
                    1
                } else {
                    0
                };

                self.v[x as usize] = self.v[x as usize].wrapping_sub(self.v[y as usize]);
            }

            Instr::SHR(x, y) => {
                /*
                TODO FOR COSMAC
                self.v[x as usize] = self.v[y as usize]
                 */

                self.v[0xF] = self.v[x as usize] & 1;
                self.v[x as usize] >>= 1;
            }

            Instr::SUBN(x, y) => {
                self.v[0xF] = if self.v[y as usize] > self.v[x as usize] {
                    1
                } else {
                    0
                };

                self.v[x as usize] = self.v[y as usize].wrapping_sub(self.v[x as usize]);
            }

            Instr::SHL(x, y) => {
                /*
                TODO FOR COSMAC
                self.v[x as usize] = self.v[y as usize]
                 */

                self.v[0xF] = self.v[x as usize] & 1;
                self.v[x as usize] <<= 1;
            }

            Instr::SNEV(x, y) => {
                if self.v[x as usize] != self.v[y as usize] {
                    self.pc += 2;
                }
            }

            Instr::LDI(nnn) => self.i = nnn,

            Instr::JPV(x, nnn) => {
                /*
                TODO NOT COSMAC
                self.pc = nnn + self.v[x as usize] as u16;
                 */
                self.pc = nnn + self.v[0] as u16;
            }

            Instr::RND(x, nn) => {
                let nbr = self.rng.gen_range(0..256) as u8;
                self.v[x as usize] = nbr & nn;
            }

            Instr::DRW(x, y, n) => {
                draw = true;
                let x_coord = (self.v[x as usize] as usize) % BUFF_WIDTH;
                let y_coord = (self.v[y as usize] as usize) % BUFF_HEIGHT;

                self.v[15] = 0;

                let mut i: usize = 0;
                let mut j: usize;
                let mut bt: u8;
                let mut bit: u8;

                while i < (n as usize) && y_coord + i < BUFF_HEIGHT {
                    bt = self.memory[(self.i as usize) + i];
                    j = 0;

                    while j < 8 && x_coord + j < BUFF_WIDTH {
                        bit = (bt >> (7 - j)) & 1;

                        if bit == 1 {
                            self.xor_bit(x_coord + j, y_coord + i, true);

                            if self.get_bit(x_coord + j, y_coord + i) {
                                self.v[15] = 1;
                            }
                        }

                        j += 1;
                    }

                    i += 1;
                }
            }

            Instr::SKP(x) => {
                if self.keys[self.v[x as usize] as usize] {
                    self.pc += 2;
                }
            }

            Instr::SKNP(x) => {
                if !self.keys[self.v[x as usize] as usize] {
                    self.pc += 2;
                }
            }

            Instr::LDXT(x) => self.v[x as usize] = self.delay_timer,

            Instr::LDK(x) => {
                //TODO implement cosmac behaviour
                if !self.keys[self.v[x as usize] as usize] {
                    self.pc -= 2;
                }
            }

            Instr::LDTX(x) => self.delay_timer = self.v[x as usize],

            Instr::LDS(x) => self.sound_timer = self.v[x as usize],

            Instr::ADDI(x) => {
                self.i += self.v[x as usize] as u16;

                //FIXME don't do if cosmac
                if self.i > 0x1000 {
                    self.v[0xF] = 1;
                }
            }

            Instr::LDF(x) => self.i = font::digit_addr(self.v[x as usize]),

            Instr::LDB(x) => {
                let nbr = self.v[x as usize];
                let hundreds = (nbr / 100) % 10;
                let tens = (nbr / 10) % 10;
                let ones = nbr % 10;

                self.memory[self.i as usize] = hundreds;
                self.memory[self.i as usize + 1] = tens;
                self.memory[self.i as usize + 2] = ones;
            }

            Instr::LDIX(x) => {
                for i in 0..=(x as usize) {
                    self.memory[self.i as usize + i] = self.v[i];
                }
                //TODO add COSMAC behaviour
            }

            Instr::LDXI(x) => {
                for i in 0..=(x as usize) {
                    self.v[i] = self.memory[self.i as usize + i];
                }
                //TODO add COSMAC behaviour
            }

            _ => (), //println!("Not yet implemented"),
        };

        draw
    }
}

// PRINT
impl Console {
    /// Prints the buffer into stdout
    pub fn print_buffer(&self) {
        // Double width to pretty print on the terminal
        println!("BUFFER");

        // Print top
        box_border::draw_top(BUFF_WIDTH * 2);

        // Print buffer
        for y in 0..BUFF_HEIGHT {
            print!("{}", box_border::VERTICAL);

            for x in 0..BUFF_WIDTH {
                let bit = self.get_bit(x, y);

                let s: &str;
                if bit {
                    s = "██";
                } else {
                    s = "  ";
                }

                print!("{}", s);
            }

            println!("{}", box_border::VERTICAL);
        }

        // Print bottom
        box_border::draw_bottom(BUFF_WIDTH * 2);
    }

    /// Prints the memory into stdout
    pub fn print_memory(&self) {
        println!("MEMORY");

        for i in 0..MEM_SIZE {
            if i % 16 == 0 {
                print!("{:#05x}: ", i);
            }

            print!("{:02x} ", self.memory[i]);

            if i % 16 == 15 {
                println!();
            }
        }
    }
}
