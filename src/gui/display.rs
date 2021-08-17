//! Contains functions to show the display of the console

use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::emu::console::*;

/// Scale of the pixels
const SCALE: usize = 20;

/// Width of the screen
const SCREEN_WIDTH: usize = BUFF_WIDTH * SCALE;
/// Height of the screen
const SCREEN_HEIGHT: usize = BUFF_HEIGHT * SCALE;

/// Display storing the canvas
pub struct Display {
    /// Canvas of the window
    pub canvas: Canvas<Window>,
}

impl Display {
    /// Creates a new display
    pub fn new(sdl_content: &sdl2::Sdl) -> Self {
        let video = sdl_content.video().unwrap();
        let window = video
            .window(
                "chip-8",
                SCREEN_WIDTH as u32,
                SCREEN_HEIGHT as u32
            )
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(pixels::Color::BLACK);
        canvas.clear();
        canvas.present();

        Display{ canvas }
    }

    /// Draws the buffer into the display
    pub fn draw(&mut self, console: &Console) {
        for y in 0..BUFF_HEIGHT {
            for x in 0..BUFF_WIDTH {
                let x_pos = x * SCALE;
                let y_pos = y * SCALE;

                let color = if console.get_bit(x, y) {
                    pixels::Color::WHITE
                } else {
                    pixels::Color::BLACK
                };

                self.canvas.set_draw_color(color);

                let rect = Rect::new(
                    x_pos as i32,
                    y_pos as i32,
                    SCALE as u32, SCALE as u32);
                self.canvas.fill_rect(rect).expect("Can't draw pixel");
            }
        }

        self.canvas.present();
    }
}