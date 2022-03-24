/// Represent a spinner and its frames.
#[derive(Clone, Debug)]
pub struct Spinner {
    /// Vector of frames representing each frame of the animation.
    pub frames: Vec<&'static str>,
    /// Interval between each frames drawing.
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
    /// Create a new spinner.
    ///
    /// # Arguments
    ///
    /// * `frames` - A vector of strings representing each frame of the animation.
    /// * `interval` - Interval between each frame drawing.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinach::Spinner;
    ///
    /// let spinner = Spinner::new(vec!["uno", "dos", "tres"], 80);
    /// ```
    pub fn new(frames: Vec<&'static str>, interval: u64) -> Self {
        Self {
            frames,
            interval,
            ..Self::default()
        }
    }
}
