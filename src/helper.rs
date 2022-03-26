#[derive(Default)]
pub struct OptText {
    pub inner: Option<String>,
}

impl From<String> for OptText {
    fn from(string: String) -> Self {
        Self {
            inner: Some(string),
        }
    }
}

impl From<&str> for OptText {
    fn from(string_slice: &str) -> Self {
        Self {
            inner: Some(string_slice.to_string()),
        }
    }
}

impl From<Option<String>> for OptText {
    fn from(inner: Option<String>) -> Self {
        Self { inner }
    }
}
