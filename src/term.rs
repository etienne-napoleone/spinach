use std::io::{stdout, Write};

/// Enum of colors supported by spinach.
#[derive(Clone)]
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

pub fn flush() {
    stdout().flush().unwrap();
}

pub fn delete_line() {
    print!("\x1b[2K")
}

pub fn hide_cursor() {
    print!("\x1b[?25l")
}

pub fn show_cursor() {
    print!("\x1b[?25h")
}

pub fn new_line() {
    println!()
}

pub(crate) fn color(color: &Color) -> Option<String> {
    match color {
        Color::Ignore => None,
        Color::Reset => Some(ansi_color(0)),
        Color::Black => Some(ansi_color(30)),
        Color::Red => Some(ansi_color(31)),
        Color::Green => Some(ansi_color(32)),
        Color::Yellow => Some(ansi_color(33)),
        Color::Blue => Some(ansi_color(34)),
        Color::Magenta => Some(ansi_color(35)),
        Color::Cyan => Some(ansi_color(36)),
        Color::White => Some(ansi_color(37)),
    }
}

fn ansi_color(code: u64) -> String {
    format!("\x1b[{}m", code)
}
