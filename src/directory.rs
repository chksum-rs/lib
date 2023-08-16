use std::fs::{DirEntry, ReadDir};
use std::io;

use crate::hash::Update;
use crate::path::PathChksumer;
use crate::{Args, Chksumer};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct DirEntryChksumer<'a, T>
where
    T: Update,
{
    pub(crate) hash: T,
    pub(crate) args: &'a Args,
}

impl<'a, T> Chksumer<DirEntry> for DirEntryChksumer<'a, T>
where
    T: Update,
{
    type Error = io::Error;

    #[inline]
    fn update(mut self, data: DirEntry) -> Result<Self, Self::Error> {
        let path = data.path();
        let Self { hash, args } = self;
        let PathChksumer { hash, args } = PathChksumer { hash, args }.update(path)?;
        self = Self { hash, args };
        Ok(self)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ReadDirChksumer<'a, T>
where
    T: Update,
{
    pub(crate) hash: T,
    pub(crate) args: &'a Args,
}

impl<'a, T> Chksumer<ReadDir> for ReadDirChksumer<'a, T>
where
    T: Update,
{
    type Error = io::Error;

    #[inline]
    fn update(mut self, data: ReadDir) -> Result<Self, Self::Error> {
        let Self { hash, args } = self;
        let DirEntryChksumer { hash, args } = {
            let mut entries = data.collect::<io::Result<Vec<DirEntry>>>()?;
            entries.sort_by_key(DirEntry::path);
            entries
                .into_iter()
                .try_fold(DirEntryChksumer { hash, args }, Chksumer::update)?
        };
        self = Self { hash, args };
        Ok(self)
    }
}
