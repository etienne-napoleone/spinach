use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::{channel, Sender, TryRecvError};
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;

use crate::state::{State, Update};
use crate::term;

/// A Spinach spinner
///
/// Represents a spinner that can be used to show progress or activity.
///
/// # Examples
///
/// ```
/// use spinach::Spinner;
///
/// let spinner = Spinner::new("Loading...").start();
/// // Perform some tasks
/// spinner.text("gg!").success();
/// ```
#[derive(Debug, Default, Clone)]
pub struct Spinner<S> {
    update: RefCell<Update>,
    state: S,
}

/// Represents the stopped state of a spinner.
#[derive(Clone, Debug)]
pub struct Stopped;

/// Represents the running state of a spinner.
#[derive(Clone, Debug)]
pub struct Running {
    sender: Sender<Update>,
    handle: Rc<RefCell<Option<JoinHandle<()>>>>,
}

/// Represents a spinner that is currently running.
pub type RunningSpinner = Spinner<Running>;

/// Represents a spinner that is currently stopped.
pub type StoppedSpinner = Spinner<Stopped>;

impl<S> Spinner<S> {
    /// Sets the color of the spinner.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinach::{Spinner, Color};
    ///
    /// let spinner = Spinner::new("workin'...").color(Color::Blue).start();
    /// ```
    pub fn color(&self, color: term::Color) -> &Self {
        self.update.borrow_mut().color = Some(color);
        self
    }

    /// Sets the text displayed alongside the spinner.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinach::Spinner;
    ///
    /// let spinner = Spinner::new("workin'...").start();
    /// ```
    pub fn text(&self, text: &str) -> &Self {
        self.update.borrow_mut().text = Some(text.to_string());
        self
    }

    /// Sets the symbols used for the spinner animation.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinach::Spinner;
    ///
    /// let spinner = Spinner::new("workin'...").symbols(vec!["◐", "◓", "◑", "◒"]).start();
    /// ```
    pub fn symbols(&self, symbols: Vec<&'static str>) -> &Self {
        self.update.borrow_mut().symbols = Some(symbols);
        self
    }

    /// Sets a single symbol for the spinner animation.
    /// This is useful when you want to set a final symbol, for example.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinach::Spinner;
    ///
    /// let spinner = Spinner::new("workin'...").start().text("done!").symbol("✔").stop();
    /// ```
    pub fn symbol(&self, symbol: &'static str) -> &Self {
        self.update.borrow_mut().symbols = Some(vec![symbol]);
        self
    }

    /// Sets the duration of each frame in the spinner animation.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinach::Spinner;
    ///
    /// let spinner = Spinner::new("workin'...").frames_duration(40).start();
    /// ```
    pub fn frames_duration(&self, ms: u64) -> &Self {
        self.update.borrow_mut().frames_duration_ms = Some(ms);
        self
    }
}

impl Spinner<Stopped> {
    /// Creates a new spinner.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinach::Spinner;
    ///
    /// let spinner = Spinner::new("let's go...").start();
    /// ```
    #[must_use]
    pub fn new(text: &str) -> Self {
        Spinner {
            update: RefCell::new(Update::new(text)),
            state: Stopped,
        }
    }

    /// Starts the spinner.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinach::Spinner;
    ///
    /// let spinner = Spinner::new("let's go...").start();
    /// ```
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
        let handle = Rc::new(handle);
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
    ///
    /// # Examples
    ///
    /// ```
    /// use spinach::Spinner;
    ///
    /// let spinner = Spinner::new("Doing something...").start();
    /// // Perform some tasks
    /// spinner.text("Doing something else...").update();
    /// ```
    pub fn update(&self) -> &Self {
        _ = self.state.sender.send(self.update.borrow().clone());
        self
    }

    /// Stops the spinner.
    ///
    /// Ideally should consume `self` but to keep a nice chaining API,
    /// it won't. Keep in mind that there's nothing to do with a
    /// spinner after it's stopped.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinach::Spinner;
    ///
    /// let spinner = Spinner::new("Doing something...").start();
    /// // Perform some tasks
    /// spinner.text("done!").stop();
    /// ```
    pub fn stop(&self) {
        self.update.borrow_mut().stop = true;
        self.update();
        self.join();
    }

    /// Stops the spinner with a pre-configured success indication.
    /// Sets the symbol and color.
    ///
    /// Ideally should consume `self` but to keep a nice chaining API,
    /// it won't. Keep in mind that there's nothing to do with a
    /// spinner after it's stopped.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinach::Spinner;
    ///
    /// let spinner = Spinner::new("Doing something...").start();
    /// // Perform some task that succeeds
    /// spinner.text("done!").success();
    /// ```
    pub fn success(&self) {
        self.update.borrow_mut().color = Some(term::Color::Green);
        self.update.borrow_mut().symbols = Some(vec!["✔"]);
        self.stop();
    }

    /// Stops the spinner with a pre-configured failure indication.
    /// Sets the symbol and color.
    ///
    /// Ideally should consume `self` but to keep a nice chaining API,
    /// it won't. Keep in mind that there's nothing to do with a
    /// spinner after it's stopped.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinach::Spinner;
    ///
    /// let spinner = Spinner::new("Doing something...").start();
    /// // Perform some task that fails
    /// spinner.text("oops").failure();
    /// ```
    pub fn failure(&self) {
        self.update.borrow_mut().color = Some(term::Color::Red);
        self.update.borrow_mut().symbols = Some(vec!["✖"]);
        self.stop();
    }

    /// Stops the spinner with a pre-configured warning indication.
    /// Sets the symbol and color.
    ///
    /// Ideally should consume `self` but to keep a nice chaining API,
    /// it won't. Keep in mind that there's nothing to do with a
    /// spinner after it's stopped.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinach::Spinner;
    ///
    /// let spinner = Spinner::new("Doing something...").start();
    /// // Perform some task with unexpected results
    /// spinner.text("wait, what?").warn();
    /// ```
    pub fn warn(&self) {
        self.update.borrow_mut().color = Some(term::Color::Yellow);
        self.update.borrow_mut().symbols = Some(vec!["⚠"]);
        self.stop();
    }
}
