use std::sync::mpsc::TryRecvError;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;

use crate::console;

pub struct SpinnerData {
    pub frames: Vec<&'static str>,
    pub interval: u64,
}

impl Default for SpinnerData {
    fn default() -> Self {
        let frames = vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let interval = 80;
        Self { frames, interval }
    }
}

struct Spinner {
    config: SpinnerData,
    text: &'static str,
    position: usize,
}

struct Frame {
    spinner: &'static str,
    text: &'static str,
}

impl Spinner {
    fn render(frame: Frame) {
        print!("\r{0} {1}", frame.spinner, frame.text);
        console::flush();
    }
}

impl Iterator for Spinner {
    type Item = Frame;

    fn next(&mut self) -> Option<Self::Item> {
        let frame = Frame {
            spinner: self.config.frames.get(self.position).unwrap(),
            text: self.text,
        };
        self.position = (self.position + 1) % self.config.frames.len();
        Some(frame)
    }
}

enum SpinnerCommand {
    Update {
        text: &'static str,
    },
    Stop {
        symbol: Option<&'static str>,
        text: Option<&'static str>,
    },
}

pub struct Spinach {
    sender: Sender<SpinnerCommand>,
    handle: thread::JoinHandle<()>,
}

impl Spinach {
    pub fn new(text: &'static str) -> Self {
        let spinner = SpinnerData::default();
        Self::run(spinner, text)
    }

    pub fn with_spinner(spinner: SpinnerData, text: &'static str) -> Self {
        Self::run(spinner, text)
    }

    pub fn stop(self, symbol: Option<&'static str>, text: Option<&'static str>) {
        self.sender
            .send(SpinnerCommand::Stop { symbol, text })
            .expect("Could not stop spinner.");
        self.handle.join().unwrap();
    }

    pub fn succeed(self, text: &'static str) {
        self.stop(Some("✔"), Some(text));
    }

    pub fn fail(self, text: &'static str) {
        self.stop(Some("✖"), Some(text));
    }

    pub fn warn(self, text: &'static str) {
        self.stop(Some("⚠"), Some(text));
    }

    pub fn info(self, text: &'static str) {
        self.stop(Some("ℹ"), Some(text));
    }

    pub fn text(&self, text: &'static str) {
        self.sender
            .send(SpinnerCommand::Update { text })
            .expect("Could not update spinner.");
    }

    fn run(config: SpinnerData, text: &'static str) -> Self {
        console::hide_cursor();

        let (sender, receiver) = channel::<SpinnerCommand>();

        let mut spinner = Spinner {
            config,
            text,
            position: 0,
        };

        let handle = thread::spawn(move || loop {
            match receiver.try_recv() {
                Ok(SpinnerCommand::Update { text }) => spinner.text = text,
                Ok(SpinnerCommand::Stop { symbol, text }) => {
                    let mut frame = spinner.next().unwrap();
                    frame.spinner = symbol.unwrap_or(frame.spinner);
                    frame.text = text.unwrap_or(frame.text);
                    Spinner::render(frame);
                    console::show_cursor();
                    break;
                }
                Err(TryRecvError::Disconnected) => {
                    Spinner::render(spinner.next().unwrap());
                    console::show_cursor();
                    break;
                }
                _ => (),
            }

            Spinner::render(spinner.next().unwrap());
            thread::sleep(Duration::from_millis(spinner.config.interval));
        });

        Self { sender, handle }
    }
}
