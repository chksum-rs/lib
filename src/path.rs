use std::fs::{read_dir, File};
use std::path::Path;

use is_terminal::IsTerminal;

use crate::directory::ReadDirChksumer;
use crate::error::Error;
use crate::hash::Update;
use crate::read::ReadChksumer;
use crate::{Args, Chksumer};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PathChksumer<'a, T>
where
    T: Update,
{
    pub(crate) hash: T,
    pub(crate) args: &'a Args,
}

impl<'a, T, U> Chksumer<U> for PathChksumer<'a, T>
where
    T: Update,
    U: AsRef<Path>,
{
    type Error = Error;

    #[inline]
    fn update(mut self, data: U) -> Result<Self, Self::Error> {
        let data = data.as_ref();
        let metadata = data.metadata()?;
        if metadata.is_dir() {
            let directory = read_dir(data)?;
            let Self { hash, args } = self;
            let ReadDirChksumer { hash, args } = ReadDirChksumer { hash, args }.update(directory)?;
            self = Self { hash, args };
        } else {
            // if not a directory then treat as a file
            let file = File::open(data)?;
            if file.is_terminal() {
                return Err(Error::IsTerminal);
            }
            let Self { hash, args } = self;
            let ReadChksumer { hash, args } = ReadChksumer { hash, args }.update(file)?;
            self = Self { hash, args };
        }
        Ok(self)
    }
}
