use std::io::{stdout, Write};

/// Spinach supported color enum.
#[derive(Clone, Debug)]
pub enum Color {
    Ignore,
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Default for Color {
    fn default() -> Self {
        Self::Cyan
    }
}

pub(crate) fn flush() {
    stdout().flush().unwrap();
}

pub(crate) fn delete_line() {
    print!("\x1b[2K");
}

pub(crate) fn hide_cursor() {
    print!("\x1b[?25l");
}

/// Print show cursor ANSI escape code
///
/// Can be used when managing ctrl^c/SIGINT to show the cursor back
///
/// # Examples
///
/// ```
/// use spinach::{Spinner, show_cursor};
///
/// let spinner = Spinner::new().text("Loading...").start();
/// // Somehow `spinner` is dropped
/// show_cursor();
/// ```
pub fn show_cursor() {
    print!("\x1b[?25h");
}

pub(crate) fn new_line() {
    println!();
}

pub(crate) fn color(color: &Color) -> String {
    match color {
        Color::Ignore => String::new(),
        Color::Reset => ansi_color(0),
        Color::Black => ansi_color(30),
        Color::Red => ansi_color(31),
        Color::Green => ansi_color(32),
        Color::Yellow => ansi_color(33),
        Color::Blue => ansi_color(34),
        Color::Magenta => ansi_color(35),
        Color::Cyan => ansi_color(36),
        Color::White => ansi_color(37),
    }
}

fn ansi_color(code: u64) -> String {
    format!("\x1b[{code}m")
}
