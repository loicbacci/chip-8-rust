//! Helpers constants and functions to draw boxes into the terminal

/// Upper left part of the box
pub const UPPER_LEFT: &str = "╭";
/// Upper right part of the box
pub const UPPER_RIGHT: &str = "╮";
/// Bottom right part of the box
pub const BOTTOM_RIGHT: &str = "╯";
/// Bottom left part of the box
pub const BOTTOM_LEFT: &str = "╰";

/// Horizontal part of the box
pub const HORIZONTAL: &str = "─";
/// Vertical part of the box
pub const VERTICAL: &str = "│";

/// Draws the top part of a box
pub fn draw_top(content_width: usize) {
    print!("{}", UPPER_LEFT);
    for _i in 0..content_width {
        print!("{}", HORIZONTAL);
    }
    println!("{}", UPPER_RIGHT);
}

/// Draws the bottom part of a box
pub fn draw_bottom(content_width: usize) {
    print!("{}", BOTTOM_LEFT);
    for _i in 0..content_width {
        print!("{}", HORIZONTAL);
    }
    println!("{}", BOTTOM_RIGHT);
}