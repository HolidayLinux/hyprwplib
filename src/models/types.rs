use std::path::Path;

pub enum Mode<'a> {
    File(&'a Path),
    Directory(&'a Vec<&'a Path>),
}
