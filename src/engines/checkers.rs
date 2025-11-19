use crate::models::{errors::NotCorrectPathError, types::Mode};

pub fn check_path(path: &str) -> Result<Mode, &dyn std::error::Error> {
    let path_entry = std::path::Path::new(path);
    if path_entry.is_file() {
        return Ok(Mode::File(path_entry));
    }
    if path_entry.is_dir() {
        let files = path_entry.read_dir();
        let only_files = files.into_iter().filter(|path| true);
        return Ok(Mode::Directory(only_files.map(|file| file)));
    }
    Err(NotCorrectPathError)
}
