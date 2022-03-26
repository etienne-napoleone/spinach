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

impl From<Option<&str>> for OptText {
    fn from(option: Option<&str>) -> Self {
        match option {
            Some(string_slice) => Self {
                inner: Some(string_slice.to_string()),
            },
            None => Self::default(),
        }
    }
}
