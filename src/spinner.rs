use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::{channel, Sender};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::term;

#[derive(Clone, Default)]
pub struct Spinner {
    update: RefCell<Update>,
    handler: Rc<RefCell<Option<Handler>>>,
}

impl Spinner {
    pub fn new() -> Self {
        Spinner::default()
    }

    pub fn text(&self, text: &str) -> &Self {
        self.update.borrow_mut().text = Some(text.to_string());
        self
    }

    pub fn color(&self) -> &Self {
        todo!()
    }

    pub fn animation(&self) -> &Self {
        todo!()
    }

    pub fn start(&self) -> Self {
        let (sender, receiver) = channel::<Command>();

        let mut context = Renderer {
            text: self.update.borrow().clone().text.unwrap_or_default(),
        };

        let handle = thread::spawn(move || loop {
            match receiver.try_recv() {
                Ok(Command::Update(update)) => context.update_with(update),
                Ok(Command::Stop(update)) => {
                    context.update_with(update);
                    context.render();
                    term::new_line();
                    break;
                }
                _ => (),
            }

            context.render();

            thread::sleep(Duration::from_millis(100));
        });

        let handler = Handler { sender, handle };

        self.handler.replace(Some(handler));
        self.update.replace(Update::default());

        self.clone()
    }

    pub fn update(&self) {
        let handler = self.handler.borrow_mut();

        match &*handler {
            Some(handler) => {
                handler
                    .sender
                    .send(Command::Update(self.update.borrow().clone()))
                    .unwrap();
            }
            None => (),
        }
    }

    pub fn stop(&self) {
        let handler = self.handler.borrow_mut().take();

        match handler {
            Some(handler) => {
                handler
                    .sender
                    .send(Command::Stop(self.update.borrow().clone()))
                    .unwrap();
                handler.handle.join().unwrap();
            }
            None => (),
        }
    }
}

struct Handler {
    sender: Sender<Command>,
    handle: JoinHandle<()>,
}

#[derive(Default)]
struct Renderer {
    text: String,
}

impl Renderer {
    pub fn render(&self) {
        term::delete_line();
        print!("\rcolor/frame/color {}", self.text);
        term::flush();
    }

    fn update_with(&mut self, update: Update) {
        if let Some(text) = update.text {
            self.text = text;
        }
    }
}

enum Command {
    Update(Update),
    Stop(Update),
}

#[derive(Clone, Default)]
struct Update {
    text: Option<String>,
}
