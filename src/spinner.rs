use std::sync::mpsc::TryRecvError;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;

use crate::console;

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

struct Context {
    spinner: Spinner,
    text: &'static str,
    color: console::Color,
}

impl Default for Context {
    fn default() -> Self {
        let spinner = Spinner::default();
        let text = "";
        let color = console::Color::Ignore;
        Self {
            spinner,
            text,
            color,
        }
    }
}

impl Context {
    fn render(&mut self) {
        Self::print(
            self.spinner.next().unwrap_or_default(),
            self.text,
            &self.color,
        );
    }

    fn render_with_override(
        &mut self,
        spinner_frame: Option<&str>,
        text: Option<&str>,
        color: Option<console::Color>,
    ) {
        let spinner_frame =
            spinner_frame.unwrap_or_else(|| self.spinner.next().unwrap_or_default());
        let text = text.unwrap_or(self.text);
        let color = color.unwrap_or_else(|| self.color.clone());
        Self::print(spinner_frame, text, &color);
    }

    fn print(spinner_frame: &str, text: &str, color: &console::Color) {
        console::delete_line();
        print!(
            "\r{}{}{} {}",
            console::color(color).unwrap_or_default(),
            spinner_frame,
            console::color(&console::Color::Reset).unwrap_or_default(),
            text,
        );
        console::flush();
    }
}

enum SpinnerCommand {
    Update {
        text: &'static str,
    },
    Stop {
        symbol: Option<&'static str>,
        text: Option<&'static str>,
        color: Option<console::Color>,
    },
}

pub struct Spinach {
    sender: Sender<SpinnerCommand>,
    handle: thread::JoinHandle<()>,
}

impl Spinach {
    pub fn new(text: &'static str) -> Self {
        let spinner = Spinner::default();
        Self::run(spinner, text, console::Color::Cyan)
    }

    pub fn with_spinner(spinner: Spinner, text: &'static str) -> Self {
        Self::run(spinner, text, console::Color::Cyan)
    }

    pub fn stop(
        self,
        symbol: Option<&'static str>,
        text: Option<&'static str>,
        color: Option<console::Color>,
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
        self.stop(Some("✔"), Some(text), Some(console::Color::Green));
    }

    pub fn fail(self, text: &'static str) {
        self.stop(Some("✖"), Some(text), Some(console::Color::Red));
    }

    pub fn warn(self, text: &'static str) {
        self.stop(Some("⚠"), Some(text), Some(console::Color::Yellow));
    }

    pub fn info(self, text: &'static str) {
        self.stop(Some("ℹ"), Some(text), Some(console::Color::Blue));
    }

    pub fn text(&self, text: &'static str) {
        self.sender
            .send(SpinnerCommand::Update { text })
            .expect("Could not update spinner.");
    }

    fn run(config: Spinner, text: &'static str, color: console::Color) -> Self {
        console::hide_cursor();

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
                    console::new_line();
                    console::show_cursor();
                    break;
                }
                Err(TryRecvError::Disconnected) => {
                    context.render();
                    console::new_line();
                    console::show_cursor();
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