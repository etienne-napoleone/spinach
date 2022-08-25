use std::io::{stdout, Write};

pub(crate) fn flush() {
    stdout().flush().unwrap();
}

pub(crate) fn delete_line() {
    print!("\x1b[2K")
}

pub(crate) fn new_line() {
    println!()
}
