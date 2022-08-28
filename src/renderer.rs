use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

use crate::color::Color;
use crate::term;

#[derive(Default)]
pub(crate) struct Renderer {
    text: String,
    color: Color,
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
        if let Some(color) = update.color {
            self.color = color;
        }
    }

    fn stop(&mut self, update: Update) {
        self.update(update);
        self.render();
        term::new_line();
    }

    fn render(&self) {
        term::delete_line();
        print!("\r{}frame{} {}", self.color, Color::Reset, self.text,);
        term::flush();
    }
}

impl From<Update> for Renderer {
    fn from(update: Update) -> Self {
        Self {
            text: update.text.unwrap_or_default(),
            color: update.color.unwrap_or_default(),
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
    pub(crate) color: Option<Color>,
}
