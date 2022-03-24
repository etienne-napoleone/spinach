use crate::spinner::Spinner;
use crate::term;

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
