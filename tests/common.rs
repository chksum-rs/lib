use std::io::Error as IoError;
use std::result;

use assert_fs::fixture::FixtureError;
use chksum::Error as ChksumError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ChksumError(#[from] ChksumError),
    #[error(transparent)]
    FixtureError(#[from] FixtureError),
    #[error(transparent)]
    IoError(#[from] IoError),
}

pub type Result = result::Result<(), Error>;
