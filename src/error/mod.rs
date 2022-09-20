use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Existing RGit project")]
    DirExists,
    #[error("Not an RGit project")]
    NotRgit,
    #[error("No path specified")]
    NoPath,
    #[error("fatal: pathspec '{0}' did not match any files")]
    NotMatchPath(String),
}
