use crate::term;

/// Represents the state of a spinner or progress indicator.
pub struct State {
    /// The text to display alongside the spinner.
    pub text: String,
    /// The color of the spinner.
    pub color: term::Color,
    /// The symbols used for animation frames.
    pub symbols: Vec<&'static str>,
    /// The duration of each frame in milliseconds.
    pub frames_duration_ms: u64,
}

impl State {
    /// Updates the state with new values from an Update struct.
    pub fn update(&mut self, update: Update) {
        if let Some(text) = update.text {
            self.text = text;
        }
        if let Some(color) = update.color {
            self.color = color;
        }
        if let Some(symbols) = update.symbols {
            self.symbols = symbols;
        }
        if let Some(frames_duration_ms) = update.frames_duration_ms {
            self.frames_duration_ms = frames_duration_ms;
        }
    }

    /// Renders the current state of the spinner.
    pub fn render(&self, iteration: usize) {
        let color = term::color(&self.color);
        let frame = self.symbols.clone()[iteration];
        let color_reset = term::color(&term::Color::Reset);
        let text = &self.text;
        term::delete_line();
        print!("\r{color}{frame}{color_reset} {text}");
        term::flush();
    }
}

impl Default for State {
    /// Default State with predefined spinner.
    fn default() -> Self {
        Self {
            text: String::new(),
            color: term::Color::default(),
            symbols: vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            frames_duration_ms: 65,
        }
    }
}

/// Represents an update to be applied to a State.
#[derive(Debug, Default, Clone)]
pub struct Update {
    /// Indicates whether to stop the spinner.
    pub stop: bool,
    /// Optional new text for the spinner.
    pub text: Option<String>,
    /// Optional new color for the spinner.
    pub color: Option<term::Color>,
    /// Optional new symbols for the spinner animation.
    pub symbols: Option<Vec<&'static str>>,
    /// Optional new frame duration in milliseconds.
    pub frames_duration_ms: Option<u64>,
}

impl Update {
    pub fn new(text: &str) -> Self {
        Self {
            text: Some(text.to_owned()),
            ..Self::default()
        }
    }
}
