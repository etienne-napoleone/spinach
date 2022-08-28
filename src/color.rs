use std::fmt::Display;

/// Spinner colors
#[derive(Clone, Default, Debug)]
pub enum Color {
    Ignore,
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    #[default]
    Cyan,
    White,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Ignore => write!(f, ""),
            Color::Reset => write!(f, "\x1b[0m"),
            Color::Black => write!(f, "\x1b[30m"),
            Color::Red => write!(f, "\x1b[31m"),
            Color::Green => write!(f, "\x1b[32m"),
            Color::Yellow => write!(f, "\x1b[33m"),
            Color::Blue => write!(f, "\x1b[34m"),
            Color::Magenta => write!(f, "\x1b[35m"),
            Color::Cyan => write!(f, "\x1b[36m"),
            Color::White => write!(f, "\x1b[37m"),
        }
    }
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "ignore" => Self::Ignore,
            "reset" => Self::Reset,
            "black" => Self::Black,
            "red" => Self::Red,
            "green" => Self::Green,
            "yellow" => Self::Yellow,
            "blue" => Self::Blue,
            "magenta" => Self::Magenta,
            "cyan" => Self::Cyan,
            "white" => Self::White,
            _ => Self::default(),
        }
    }
}
