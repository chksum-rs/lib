use std::{io, result};

/// A common error type for the current crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cannot process terminal input")]
    IsTerminal,
    #[error(transparent)]
    Io(#[from] io::Error),
}

/// [`Result`](std::result::Result) type alias with an error type of [`Error`].
pub type Result<T> = result::Result<T, Error>;
