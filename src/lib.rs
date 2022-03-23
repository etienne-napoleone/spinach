use std::sync::mpsc::{channel, Sender, TryRecvError};
use std::thread;
use std::time::Duration;

use spinner::{Context, SpinnerCommand};

pub use spinner::Spinner;
pub use term::Color;

mod spinner;
mod term;

pub struct Spinach {
    sender: Sender<SpinnerCommand>,
    handle: thread::JoinHandle<()>,
}

impl Spinach {
    pub fn new(text: &'static str) -> Self {
        let spinner = Spinner::default();
        Self::run(spinner, text, term::Color::Cyan)
    }

    pub fn with_spinner(spinner: Spinner, text: &'static str) -> Self {
        Self::run(spinner, text, term::Color::Cyan)
    }

    pub fn stop(
        self,
        symbol: Option<&'static str>,
        text: Option<&'static str>,
        color: Option<term::Color>,
    ) {
        self.sender
            .send(SpinnerCommand::Stop {
                symbol,
                text,
                color,
            })
            .expect("Could not stop spinner.");
        self.handle.join().unwrap();
    }

    pub fn succeed(self, text: &'static str) {
        self.stop(Some("✔"), Some(text), Some(term::Color::Green));
    }

    pub fn fail(self, text: &'static str) {
        self.stop(Some("✖"), Some(text), Some(term::Color::Red));
    }

    pub fn warn(self, text: &'static str) {
        self.stop(Some("⚠"), Some(text), Some(term::Color::Yellow));
    }

    pub fn info(self, text: &'static str) {
        self.stop(Some("ℹ"), Some(text), Some(term::Color::Blue));
    }

    pub fn text(&self, text: &'static str) {
        self.sender
            .send(SpinnerCommand::Update { text })
            .expect("Could not update spinner.");
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
                Ok(SpinnerCommand::Update { text }) => context.text = text,
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
