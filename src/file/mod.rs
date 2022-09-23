use std::fmt::Display;

#[derive(PartialEq, Eq, Debug)]
pub enum Mode {
    Blob,
    Dir,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Blob => write!(f, "blob"),
            Mode::Dir => write!(f, "dir"),
        }
    }
}
