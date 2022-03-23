use crate::term;

pub struct Spinner {
    pub frames: Vec<&'static str>,
    pub interval: u64,
    position: usize,
}

impl Default for Spinner {
    fn default() -> Self {
        let frames = vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let interval = 80;
        Self {
            frames,
            interval,
            position: 0,
        }
    }
}

impl Iterator for Spinner {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        let frame = self.frames.get(self.position).unwrap();
        self.position = (self.position + 1) % self.frames.len();
        Some(frame)
    }
}

impl Spinner {
    pub fn new(frames: Vec<&'static str>, interval: u64) -> Self {
        Self {
            frames,
            interval,
            ..Self::default()
        }
    }
}

pub(crate) struct Context {
    pub spinner: Spinner,
    pub text: &'static str,
    pub color: term::Color,
}

impl Default for Context {
    fn default() -> Self {
        let spinner = Spinner::default();
        let text = "";
        let color = term::Color::Ignore;
        Self {
            spinner,
            text,
            color,
        }
    }
}

impl Context {
    pub fn render(&mut self) {
        Self::print(
            self.spinner.next().unwrap_or_default(),
            self.text,
            &self.color,
        );
    }

    pub fn render_with_override(
        &mut self,
        spinner_frame: Option<&str>,
        text: Option<&str>,
        color: Option<term::Color>,
    ) {
        let spinner_frame =
            spinner_frame.unwrap_or_else(|| self.spinner.next().unwrap_or_default());
        let text = text.unwrap_or(self.text);
        let color = color.unwrap_or_else(|| self.color.clone());
        Self::print(spinner_frame, text, &color);
    }

    pub fn print(spinner_frame: &str, text: &str, color: &term::Color) {
        term::delete_line();
        print!(
            "\r{}{}{} {}",
            term::color(color).unwrap_or_default(),
            spinner_frame,
            term::color(&term::Color::Reset).unwrap_or_default(),
            text,
        );
        term::flush();
    }
}

pub(crate) enum SpinnerCommand {
    Update {
        text: &'static str,
    },
    Stop {
        symbol: Option<&'static str>,
        text: Option<&'static str>,
        color: Option<term::Color>,
    },
}
