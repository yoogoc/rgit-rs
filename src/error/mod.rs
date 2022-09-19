use thiserror::Error;

#[derive(Error, Debug)]
pub enum InitError {
    #[error("Existing RGit project")]
    DirExists(),
}
