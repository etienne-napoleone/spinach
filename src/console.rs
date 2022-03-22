use std::io::{stdout, Write};

pub fn flush() {
    stdout().flush().unwrap();
}

pub fn hide_cursor() {
    print!("\x1b[?25l")
}

pub fn show_cursor() {
    print!("\x1b[?25h")
}
