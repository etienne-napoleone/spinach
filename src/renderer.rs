use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

use crate::term;

#[derive(Default)]
pub(crate) struct Renderer {
    text: String,
}

impl Renderer {
    pub fn start(&mut self, receiver: Receiver<Command>) {
        loop {
            match receiver.try_recv() {
                Ok(Command::Update(update)) => self.update(update),
                Ok(Command::Stop(update)) => {
                    self.stop(update);
                    break;
                }
                _ => (),
            }

            self.render();

            thread::sleep(Duration::from_millis(100));
        }
    }

    fn update(&mut self, update: Update) {
        if let Some(text) = update.text {
            self.text = text;
        }
    }

    fn stop(&mut self, update: Update) {
        self.update(update);
        self.render();
        term::new_line();
    }

    fn render(&self) {
        term::delete_line();
        print!("\rcolor/frame/color {}", self.text);
        term::flush();
    }
}

impl From<Update> for Renderer {
    fn from(update: Update) -> Self {
        Self {
            text: update.text.unwrap_or_default(),
        }
    }
}

pub(crate) enum Command {
    Update(Update),
    Stop(Update),
}

#[derive(Clone, Default)]
pub(crate) struct Update {
    pub(crate) text: Option<String>,
}
