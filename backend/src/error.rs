use std::error::Error as StdError;
use std::fmt::Display;

/// Custom error for the crate.
#[derive(Debug)]
pub struct Error {
    /// Message included in the error.
    msg: String,
    src: Option<Box<dyn StdError>>,
}

impl Error {
    /// Returns an [Error] instance.
    ///
    /// # Arguments
    ///
    /// * `msg` - [Error] message.
    ///
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_owned(),
            src: None,
        }
    }

    /// Returns an [Error] instance.
    ///
    /// # Arguments
    ///
    /// * `msg` - [Error] message.
    /// * `src` - Error source
    ///
    pub fn new_with_source<T>(msg: &str, src: T) -> Self
    where
        T: StdError + 'static,
    {
        Self {
            msg: msg.to_owned(),
            src: Some(Box::new(src)),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.msg)
    }
}

impl StdError for Error {
    fn source<'a>(&'a self) -> Option<&'a (dyn StdError + 'static)> {
        match self.src.as_ref() {
            None => None,
            Some(src) => Some(src.as_ref()),
        }
    }
}
