use std::fmt::Display;

#[derive(Debug, Clone, Display)]
pub struct NotCorrectPathError;

impl Display for NotCorrectPathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut m = Vec::new();
        write!(&mut m, "Not correct path.")
    }
}
