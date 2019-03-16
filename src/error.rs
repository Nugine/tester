use std::fmt::Display;

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(msg: &str) -> Self {
        Self {
            message: msg.into(),
        }
    }

    pub fn msg(&self) -> &str {
        &self.message
    }
}

impl<T: Display> From<T> for Error {
    fn from(error: T) -> Self {
        Self {
            message: format!("{}", error),
        }
    }
}
