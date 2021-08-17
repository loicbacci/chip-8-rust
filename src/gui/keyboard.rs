//! Contains functions used to interface with the keyboard

use sdl2::EventPump;
use crate::emu::console::Console;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

/// Represents the keyboard
/// Contains the EventPump
pub struct Keyboard {
    /// The event pump of the ui
    event_pump: EventPump,
}

impl Keyboard {
    /// Creates a new keyboard
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let event_pump = sdl_context.event_pump().unwrap();
        Keyboard{ event_pump }
    }

    /// Polls the keys from the keyboard and inputs them into console
    /// Returns true if the program continues
    pub fn poll_keys(&mut self, console: &mut Console, debug: bool) -> bool {
        let mut cont = true;
        for event in self.event_pump.poll_iter() {
            let key_index: Option<usize> = match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    cont = false;
                    break;
                },
                Event::KeyUp {keycode, ..} |
                Event::KeyDown { keycode, .. } => {
                    let key = keycode.unwrap();
                    match key {
                        Keycode::Num1 => Some(0x1),
                        Keycode::Num2 => Some(0x2),
                        Keycode::Num3 => Some(0x3),
                        Keycode::Num4 => Some(0xC),

                        Keycode::Q => Some(0x4),
                        Keycode::W => Some(0x5),
                        Keycode::E => Some(0x6),
                        Keycode::R => Some(0xD),

                        Keycode::A => Some(0x7),
                        Keycode::S => Some(0x8),
                        Keycode::D => Some(0x9),
                        Keycode::F => Some(0xE),

                        Keycode::Y => Some(0xA),
                        Keycode::X => Some(0x0),
                        Keycode::C => Some(0xB),
                        Keycode::V => Some(0xF),

                        _ => None,
                    }
                }
                _ => None,
            };

            let down = match event {
                Event::KeyUp {..} => false,
                Event::KeyDown {..} => true,
                _ => false,
            };

            // PRESS KEY
            if let Some(i) = key_index {
                if debug {
                    println!("Key {:x} is {}", i,
                        if down { "down" } else { "up" }
                    );
                }
                console.set_key(i, down)
            };
        }

        cont
    }
}