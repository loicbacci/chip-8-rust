//! Contains the font of the console

/// Base address in the memory for the font
pub const FONT_ADDR: u16 = 0x50;

/// Returns the memory address of a given digit
pub fn digit_addr(digit: u8) -> u16 {
    FONT_ADDR + digit as u16 * 5
}

/// Font of the console (digits from 0 to F)
pub const FONT: [[u8; 5]; 16] = [
    F0, F1, F2, F3, F4, F5, F6, F7, F8, F9,
    FA, FB, FC, FD, FE, FF,
];

/// Digit 0
const F0: [u8; 5] = [
    0xF0,
    0x90,
    0x90,
    0x90,
    0xF0,
];

/// Digit 1
const F1: [u8; 5] = [
    0x20,
    0x60,
    0x20,
    0x20,
    0x70,
];

/// Digit 2
const F2: [u8; 5] = [
    0xF0,
    0x10,
    0xF0,
    0x80,
    0xF0,
];

/// Digit 3
const F3: [u8; 5] = [
    0xF0,
    0x10,
    0xF0,
    0x10,
    0xF0,
];

/// Digit 4
const F4: [u8; 5] = [
    0x90,
    0x90,
    0xF0,
    0x10,
    0x10,
];

/// Digit 5
const F5: [u8; 5] = [
    0xF0,
    0x80,
    0xF0,
    0x10,
    0xF0,
];

/// Digit 6
const F6: [u8; 5] = [
    0xF0,
    0x80,
    0xF0,
    0x90,
    0xF0,
];

/// Digit 7
const F7: [u8; 5] = [
    0xF0,
    0x10,
    0x20,
    0x40,
    0x40,
];

/// Digit 8
const F8: [u8; 5] = [
    0xF0,
    0x90,
    0xF0,
    0x90,
    0xF0,
];

/// Digit 9
const F9: [u8; 5] = [
    0xF0,
    0x90,
    0xF0,
    0x10,
    0xF0,
];

/// Digit A
const FA: [u8; 5] = [
    0xF0,
    0x90,
    0xF0,
    0x90,
    0x90,
];

/// Digit B
const FB: [u8; 5] = [
    0xE0,
    0x90,
    0xE0,
    0x90,
    0xE0,
];

/// Digit C
const FC: [u8; 5] = [
    0xF0,
    0x80,
    0x80,
    0x80,
    0xF0,
];

/// Digit D
const FD: [u8; 5] = [
    0xE0,
    0x90,
    0x90,
    0x90,
    0xE0,
];

/// Digit E
const FE: [u8; 5] = [
    0xF0,
    0x80,
    0xF0,
    0x80,
    0xF0,
];

/// Digit F
const FF: [u8; 5] = [
    0xF0,
    0x80,
    0xF0,
    0x80,
    0x80,
];