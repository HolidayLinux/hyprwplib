use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub struct NotCorrectPathError;

impl Error for NotCorrectPathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Not correct path.")
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }

    fn provide<'a>(&'a self, request: &mut std::error::Request<'a>) {}
}
