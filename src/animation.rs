#[derive(Clone, Debug)]
pub struct Animation {
    pub frames: Vec<&'static str>,
    pub interval: u64,
    position: usize,
}

impl Default for Animation {
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

impl Iterator for Animation {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        let frame = self.frames.get(self.position).unwrap();
        self.position = (self.position + 1) % self.frames.len();
        Some(frame)
    }
}
