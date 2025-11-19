use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub struct NotCorrectPathError;

impl Error for NotCorrectPathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Not correct path.")
    }
}
