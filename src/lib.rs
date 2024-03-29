use std::sync::mpsc::{channel, Sender, TryRecvError};
use std::thread;
use std::time::Duration;

use crate::context::Context;
use crate::helper::OptText;

pub use crate::spinner::Spinner;
pub use crate::term::Color;

mod context;
mod helper;
mod spinner;

pub mod term;

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
    pub fn new_with<A: Into<Option<Spinner>>, B: Into<OptText>, C: Into<Option<term::Color>>>(
        spinner: A,
        text: B,
        color: C,
    ) -> Self {
        Self::run(
            spinner.into().unwrap_or_default(),
            text.into().inner.unwrap_or_default(),
            color.into().unwrap_or_default(),
        )
    }

    /// Create a new spinach spinner using defaults and passed text.
    ///
    /// # Arguments
    ///
    /// * `text` - Optional spinner text.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinach::Spinach;
    ///
    /// let s = Spinach::new("hey hey");
    /// ```
    pub fn new<A: Into<OptText>>(text: A) -> Self {
        let spinner = Spinner::default();
        Self::run(
            spinner,
            text.into().inner.unwrap_or_default(),
            term::Color::Cyan,
        )
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
    pub fn update_with<A: Into<OptText>, B: Into<Option<term::Color>>>(&self, text: A, color: B) {
        self.sender
            .send(SpinnerCommand::Update {
                text: text.into().inner,
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
    pub fn text<A: Into<String>>(&self, text: A) {
        self.update_with(text.into(), None);
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
        self.update_with(OptText::default(), color);
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
        B: Into<OptText>,
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
                text: text.into().inner,
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
    pub fn succeed<A: Into<OptText>>(self, text: A) {
        self.stop_with("✔", text.into(), term::Color::Green);
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
    pub fn fail<A: Into<OptText>>(self, text: A) {
        self.stop_with("✖", text.into(), term::Color::Red);
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
    pub fn warn<A: Into<OptText>>(self, text: A) {
        self.stop_with("⚠", text.into(), term::Color::Yellow);
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
    pub fn info<A: Into<OptText>>(self, text: A) {
        self.stop_with("ℹ", text.into(), term::Color::Blue);
    }

    /// Freeze spinach spinner with passed optional configurations and
    /// continue on the next line.
    ///
    /// # Arguments
    ///
    /// When `None`, use current value.
    ///
    /// * `frozen_symbol` - Optional symbol used as the spinner's frozen line frame.
    /// * `frozen_text` - Optional spinner's frozen line text.
    /// * `frozen_color` - Optional spinner's frozen line spinner color.
    /// * `text` - Optional spinner new line text.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinach::{Color, Spinach};
    ///
    /// let s = Spinach::new("first task");
    /// s.freeze("✔", "first task: ok", Color::Green, "second task");
    /// s.stop_with("✔", "first task: ok", Color::Green);
    /// ```
    pub fn freeze<
        A: Into<Option<&'static str>>,
        B: Into<OptText>,
        C: Into<Option<term::Color>>,
        D: Into<OptText>,
    >(
        &self,
        frozen_symbol: A,
        frozen_text: B,
        frozen_color: C,
        text: D,
    ) {
        self.sender
            .send(SpinnerCommand::Freeze {
                frozen_symbol: frozen_symbol.into(),
                frozen_text: frozen_text.into().inner,
                frozen_color: frozen_color.into(),
                text: text.into().inner,
            })
            .expect("Could not stop spinner.");
    }

    fn run(config: Spinner, text: String, color: term::Color) -> Self {
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
                Ok(SpinnerCommand::Freeze {
                    frozen_symbol,
                    frozen_text,
                    frozen_color,
                    text,
                }) => {
                    context.render_with_override(frozen_symbol, frozen_text, frozen_color);
                    term::new_line();
                    if let Some(text) = text {
                        context.text = text;
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

enum SpinnerCommand {
    Update {
        text: Option<String>,
        color: Option<term::Color>,
    },
    Freeze {
        frozen_symbol: Option<&'static str>,
        frozen_text: Option<String>,
        frozen_color: Option<term::Color>,
        text: Option<String>,
    },
    Stop {
        symbol: Option<&'static str>,
        text: Option<String>,
        color: Option<term::Color>,
    },
}
