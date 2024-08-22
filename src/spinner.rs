use std::cell::RefCell;
use std::sync::mpsc::{channel, Sender, TryRecvError};
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;

use crate::state::{State, Update};
use crate::term;

/// A Spinach spinner
#[derive(Debug, Default, Clone)]
pub struct Spinner<S> {
    update: RefCell<Update>,
    state: S,
}

/// Represents the stopped state of a spinner.
#[derive(Debug)]
pub struct Stopped;

/// Represents the running state of a spinner.
#[derive(Debug)]
pub struct Running {
    sender: Sender<Update>,
    handle: RefCell<Option<JoinHandle<()>>>,
}

impl<S> Spinner<S> {
    /// Sets the color of the spinner.
    pub fn color(&self, color: term::Color) -> &Self {
        self.update.borrow_mut().color = Some(color);
        self
    }

    /// Sets the text displayed alongside the spinner.
    pub fn text(&self, text: &str) -> &Self {
        self.update.borrow_mut().text = Some(text.to_string());
        self
    }

    /// Sets the symbols used for the spinner animation.
    pub fn symbols(&self, symbols: Vec<&'static str>) -> &Self {
        self.update.borrow_mut().symbols = Some(symbols);
        self
    }

    /// Sets the duration of each frame in the spinner animation.
    pub fn frames_duration(&self, ms: u64) -> &Self {
        self.update.borrow_mut().frames_duration_ms = Some(ms);
        self
    }
}

impl Spinner<Stopped> {
    /// Creates a new spinner.
    #[must_use]
    pub fn new() -> Self {
        Spinner {
            update: RefCell::new(Update::default()),
            state: Stopped,
        }
    }

    /// Starts the spinner.
    pub fn start(&self) -> Spinner<Running> {
        term::hide_cursor();
        let (sender, receiver) = channel::<Update>();
        let mut state = State::default();
        state.update(self.update.take());
        let handle = RefCell::new(Some(spawn(move || {
            let mut iteration = 0;
            loop {
                match receiver.try_recv() {
                    Ok(update) if update.stop => {
                        state.update(update);
                        if iteration >= state.symbols.len() {
                            iteration = 0;
                        }
                        state.render(iteration);
                        break;
                    }
                    Ok(update) => state.update(update),
                    Err(TryRecvError::Disconnected) => break,
                    Err(TryRecvError::Empty) => (),
                }
                if iteration >= state.symbols.len() {
                    iteration = 0;
                }
                state.render(iteration);
                iteration += 1;
                sleep(Duration::from_millis(state.frames_duration_ms));
            }
            term::new_line();
            term::show_cursor();
        })));

        Spinner {
            update: RefCell::new(Update::default()),
            state: Running { sender, handle },
        }
    }
}

impl Spinner<Running> {
    /// Joins the spinner thread, stopping it.
    fn join(&self) {
        if let Some(handle) = self.state.handle.borrow_mut().take() {
            _ = handle.join();
        }
    }

    /// Updates the spinner with the current update state.
    pub fn update(&self) -> &Self {
        _ = self.state.sender.send(self.update.borrow().clone());
        self
    }

    /// Stops the spinner.
    pub fn stop(&self) {
        self.update.borrow_mut().stop = true;
        self.update();
        self.join();
    }

    /// Stops the spinner with a success indication.
    pub fn success(&self) {
        self.update.borrow_mut().color = Some(term::Color::Green);
        self.update.borrow_mut().symbols = Some(vec!["✔"]);
        self.stop();
    }

    /// Stops the spinner with a failure indication.
    pub fn failure(&self) {
        self.update.borrow_mut().color = Some(term::Color::Red);
        self.update.borrow_mut().symbols = Some(vec!["✖"]);
        self.stop();
    }

    /// Stops the spinner with a warning indication.
    pub fn warn(&self) {
        self.update.borrow_mut().color = Some(term::Color::Yellow);
        self.update.borrow_mut().symbols = Some(vec!["⚠"]);
        self.stop();
    }
}
