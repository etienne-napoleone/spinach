use std::sync::mpsc::{channel, Sender, TryRecvError};
use std::thread;
use std::time::Duration;

use spinner::{Context, SpinnerCommand};

pub use spinner::Spinner;
pub use term::Color;

mod spinner;
mod term;

/// Spinach spinner.
///
/// # Examples
///
/// ```
/// use spinach::Spinach;
///
/// let s = Spinach::new("Cutting spinaches");
/// // do something long
/// s.succeed("Cut spinaches");
/// ```
pub struct Spinach {
    sender: Sender<SpinnerCommand>,
    handle: thread::JoinHandle<()>,
}

impl Spinach {
    /// Create a new spinach spinner with passed full configuration.
    ///
    /// # Arguments
    ///
    /// When `None`, will use default value.
    ///
    /// * `spinner` - Optional spinner object.
    /// * `text` - Optional spinner text.
    /// * `color` - Optional spinner color.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinach::{Color, Spinach, Spinner};
    ///
    /// let spinner = Spinner::new(vec!["uno", "dos", "tres"], 80);
    /// let s = Spinach::new_with(spinner, "hey hey", Color::Yellow);
    /// ```
    pub fn new_with<
        A: Into<Option<Spinner>>,
        B: Into<Option<&'static str>>,
        C: Into<Option<term::Color>>,
    >(
        spinner: A,
        text: B,
        color: C,
    ) -> Self {
        Self::run(
            spinner.into().unwrap_or_default(),
            text.into().unwrap_or_default(),
            color.into().unwrap_or_default(),
        )
    }

    /// Create a new spinach spinner using defaults and passed text.
    ///
    /// # Arguments
    ///
    /// * `text` - Spinner text.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinach::Spinach;
    ///
    /// let s = Spinach::new("hey hey");
    /// ```
    pub fn new(text: &'static str) -> Self {
        let spinner = Spinner::default();
        Self::run(spinner, text, term::Color::Cyan)
    }

    /// Update spinach spinner with passed optional configurations.
    ///
    /// # Arguments
    ///
    /// When `None`, use default value.
    ///
    /// * `text` - Optional spinner text.
    /// * `color` - Optional spinner color.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinach::{Color, Spinach};
    ///
    /// let s = Spinach::new("hey hey");
    /// s.update_with(None, Color::Red);
    pub fn update_with<A: Into<Option<&'static str>>, B: Into<Option<term::Color>>>(
        &self,
        text: A,
        color: B,
    ) {
        self.sender
            .send(SpinnerCommand::Update {
                text: text.into(),
                color: color.into(),
            })
            .expect("Could not update spinner.");
    }

    /// Update spinach spinner text.
    ///
    /// # Arguments
    ///
    /// * `text` - Spinner text.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinach::Spinach;
    ///
    /// let s = Spinach::new("hey hey");
    /// s.text("hi hi");
    /// ```
    pub fn text(&self, text: &'static str) {
        self.update_with(text, None);
    }

    /// Update spinach spinner color
    ///
    /// # Arguments
    ///
    /// * `color` - Spinner color.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinach::{Color, Spinach};
    ///
    /// let s = Spinach::new("hey hey");
    /// s.color(Color::Red);
    /// ```
    pub fn color(&self, color: term::Color) {
        self.update_with(None, color);
    }

    /// Stop spinach spinner with passed optional configurations.
    ///
    /// # Arguments
    ///
    /// When `None`, use current value.
    ///
    /// * `symbol` - Optional symbol used as the spinner's final frame.
    /// * `text` - Optional spinner text.
    /// * `color` - Optional spinner color.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinach::{Color, Spinach};
    ///
    /// let s = Spinach::new("hey hey");
    /// s.stop_with("✔", None, Color::Green);
    /// ```
    pub fn stop_with<
        A: Into<Option<&'static str>>,
        B: Into<Option<&'static str>>,
        C: Into<Option<term::Color>>,
    >(
        self,
        symbol: A,
        text: B,
        color: C,
    ) {
        self.sender
            .send(SpinnerCommand::Stop {
                symbol: symbol.into(),
                text: text.into(),
                color: color.into(),
            })
            .expect("Could not stop spinner.");
        self.handle.join().unwrap();
    }

    /// Stop spinach spinner without additional changes.
    ///
    /// Usually not what you are looking for, spinner will appear as frozen
    /// due to the lack of final frame.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinach::Spinach;
    ///
    /// let s = Spinach::new("hey hey");
    /// s.stop();
    /// ```
    pub fn stop(self) {
        self.stop_with(None, None, None);
    }

    /// Stop spinach spinner with final `✔` frame and green color.
    ///
    /// # Arguments
    ///
    /// When `None`, use current value.
    ///
    /// * `text` - Optional spinner text.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinach::Spinach;
    ///
    /// let s = Spinach::new("hey hey");
    /// s.succeed("ok");
    /// ```
    pub fn succeed<A: Into<Option<&'static str>>>(self, text: A) {
        self.stop_with("✔", text, term::Color::Green);
    }

    /// Stop spinach spinner with final `✖` frame and red color.
    ///
    /// # Arguments
    ///
    /// When `None`, use current value.
    ///
    /// * `text` - Optional spinner text.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinach::Spinach;
    ///
    /// let s = Spinach::new("hey hey");
    /// s.fail("nok");
    /// ```
    pub fn fail<A: Into<Option<&'static str>>>(self, text: A) {
        self.stop_with("✖", text, term::Color::Red);
    }

    /// Stop spinach spinner with final `⚠` frame and yellow color.
    ///
    /// # Arguments
    ///
    /// When `None`, use current value.
    ///
    /// * `text` - Optional spinner text.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinach::Spinach;
    ///
    /// let s = Spinach::new("hey hey");
    /// s.warn("warning");
    /// ```
    pub fn warn<A: Into<Option<&'static str>>>(self, text: A) {
        self.stop_with("⚠", text, term::Color::Yellow);
    }

    /// Stop spinach spinner with final `ℹ` frame and blue color.
    ///
    /// # Arguments
    ///
    /// When `None`, use current value.
    ///
    /// * `text` - Optional spinner text.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinach::Spinach;
    ///
    /// let s = Spinach::new("hey hey");
    /// s.info("done");
    /// ```
    pub fn info<A: Into<Option<&'static str>>>(self, text: A) {
        self.stop_with("ℹ", text, term::Color::Blue);
    }

    fn run(config: Spinner, text: &'static str, color: term::Color) -> Self {
        term::hide_cursor();

        let (sender, receiver) = channel::<SpinnerCommand>();

        let mut context = Context {
            spinner: config,
            text,
            color,
        };

        let handle = thread::spawn(move || loop {
            match receiver.try_recv() {
                Ok(SpinnerCommand::Update { text, color }) => {
                    if let Some(text) = text {
                        context.text = text;
                    }
                    if let Some(color) = color {
                        context.color = color;
                    }
                }
                Ok(SpinnerCommand::Stop {
                    symbol,
                    text,
                    color,
                }) => {
                    context.render_with_override(symbol, text, color);
                    term::new_line();
                    term::show_cursor();
                    break;
                }
                Err(TryRecvError::Disconnected) => {
                    context.render();
                    term::new_line();
                    term::show_cursor();
                    break;
                }
                _ => (),
            }

            context.render();
            thread::sleep(Duration::from_millis(context.spinner.interval));
        });

        Self { sender, handle }
    }
}
