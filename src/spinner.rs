use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::{channel, Sender};
use std::thread::{self, JoinHandle};

use crate::color::Color;
use crate::renderer::{Command, Renderer, Update};

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

    pub fn color<C: Into<Color>>(&self, color: C) -> &Self {
        self.update.borrow_mut().color = Some(color.into());
        self
    }

    pub fn animation(&self) -> &Self {
        todo!()
    }

    pub fn start(&self) -> Self {
        let (sender, receiver) = channel::<Command>();

        let update = self.update.replace(Update::default());
        let mut renderer = Renderer::from(update);

        let handle = thread::spawn(move || renderer.start(receiver));
        self.handler.replace(Some(Handler { sender, handle }));

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
