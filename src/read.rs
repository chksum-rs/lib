use std::io::{BufRead, BufReader, Read};

use crate::error::Error;
use crate::hash::Update;
use crate::{Args, Chksumer};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ReadChksumer<'a, T>
where
    T: Update,
{
    pub(crate) hash: T,
    pub(crate) args: &'a Args,
}

impl<'a, T, U> Chksumer<U> for ReadChksumer<'a, T>
where
    T: Update,
    U: Read,
{
    type Error = Error;

    fn update(mut self, data: U) -> Result<Self, Self::Error> {
        let Self { mut hash, args } = self;
        let mut data = match args.chunk_size {
            Some(chunk_size) => BufReader::with_capacity(chunk_size, data),
            None => BufReader::new(data),
        };
        loop {
            let buffer = data.fill_buf()?;
            let length = buffer.len();
            if length == 0 {
                break;
            }
            hash = hash.update(buffer);
            data.consume(length);
        }
        self = Self { hash, args };
        Ok(self)
    }
}
