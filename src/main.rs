#![warn(clippy::missing_docs_in_private_items)]
#![allow(clippy::many_single_char_names)]

//! Chip-8 emulator written in rust

extern crate sdl2;

use crate::emu::console::*;
use crate::gui::display::*;
use std::thread;
use std::time::Duration;
use crate::gui::keyboard::Keyboard;
use crate::gui::sound::Sound;

mod emu;
mod gui;

fn main() {
    println!("Chip-8 emulator");

    //TODO ability to change FPS
    /// "Speed" of the emulation
    const FPS: u64 = 60;

    let filename = std::env::args()
        .nth(1)
        .expect("no filename given");

    let mut cons: Console = Console::new();
    cons.load_rom(filename.as_str());

    let sdl_context = sdl2::init().unwrap();

    let mut display = Display::new(&sdl_context);
    let mut keyboard = Keyboard::new(&sdl_context);
    let sound = Sound::new(&sdl_context);

    let mut running = true;

    while running {
        running = keyboard.poll_keys(&mut cons, true);

        let (draw, play_sound) = cons.cycle(false, false);
        if draw { display.draw(&cons) };

        if play_sound {
            sound.start_beep();
        } else {
            sound.stop_beep();
        }

        thread::sleep(Duration::from_millis(1000 / FPS));
    }
}
