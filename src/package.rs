use std::path::Path;

// TODO: implement fmt::Display
#[derive(Debug)]
pub enum Package {
    Pkg(String),
    Path(Box<Path>),
}
