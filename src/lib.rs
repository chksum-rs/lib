//! High-level interface for easy calculation of checksum digest for files, directories, stdin and more.
//!
//! # Setup
//!
//! Update your `Cargo.toml` by adding entry to `dependencies` section.
//!
//! ```toml
//! [dependencies]
//! # ...
//! chksum = "0.2.0"
//! ```
//!
//! Alternatively use [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand.
//!
//! ```sh
//! cargo add chksum
//! ```
//!
//! # Usage
//!
//! ## File
//!
//! Use [`File`](std::fs::File) or [`&File`](std::fs::File) as an input.
//!
//! ```rust
//! # use std::fs::File;
//! # use std::path::Path;
//! # use chksum::Result;
//! use chksum::chksum;
//! use chksum::hash::MD5;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let file = File::open(path)?;
//! let digest = chksum::<MD5, _>(file)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "b35e02f32d924c3da7ca8613ea91deb0"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## Directory
//!
//! Use [`ReadDir`](std::fs::ReadDir) as an input.
//!
//! ```rust
//! # use std::fs::read_dir;
//! # use std::path::Path;
//! # use chksum::Result;
//! use chksum::chksum;
//! use chksum::hash::MD5;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let dir = read_dir(path)?;
//! let digest = chksum::<MD5, _>(dir)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "30672e8a0cb95ef3b0a29601f2874ba5"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## Paths
//!
//! Use [`&Path`](std::path::Path), [`PathBuf`](std::path::PathBuf) or [`&PathBuf`](std::path::PathBuf) as an input.
//!
//! ```rust
//! # use std::path::{Path, PathBuf};
//! # use chksum::Result;
//! use chksum::chksum;
//! use chksum::hash::MD5;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let digest = chksum::<MD5, _>(path)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "0bccc59d6997a74e1813d058aa1ad80d"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## Stdin
//!
//! Use [`StdinLock`](std::io::StdinLock) as an input.
//!
//! ```rust
//! # use std::io::stdin;
//! # use chksum::Result;
//! use chksum::chksum;
//! use chksum::hash::MD5;
//!
//! # fn wrapper() -> Result<()> {
//! let handle = stdin().lock();
//! let digest = chksum::<MD5, _>(handle)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "e91509789cba933399182e6a8864bdd8"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! # Algorithms
//!
//! ## MD5
//!
//! Use [`MD5`](hash::MD5) struct to calculate MD5 digest.
//!
//! ```rust
//! # use std::fs::File;
//! # use std::path::Path;
//! # use chksum::Result;
//! use chksum::chksum;
//! use chksum::hash::MD5;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let digest = chksum::<MD5, _>(path)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "3081d73d94e101bfa7bf39a3ef7351e9"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## SHA-1
//!
//! Use [`SHA1`](hash::SHA1) struct to calculate SHA-1 digest.
//!
//! ```rust
//! # use std::fs::File;
//! # use std::path::Path;
//! # use chksum::Result;
//! use chksum::chksum;
//! use chksum::hash::SHA1;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let digest = chksum::<SHA1, _>(path)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "d2b8d92228efb73147151566f059d1cace37046e"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## SHA-2
//!
//! ### SHA-2 224
//!
//! Use [`SHA2_224`](hash::SHA2_224) struct to calculate SHA-2 224 digest.
//!
//! ```rust
//! # use std::fs::File;
//! # use std::path::Path;
//! # use chksum::Result;
//! use chksum::chksum;
//! use chksum::hash::SHA2_224;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let digest = chksum::<SHA2_224, _>(path)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "cfa726b42c9ef788e99eb81e4dce181763feac230485dde49dba717a"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ### SHA-2 256
//!
//! Use [`SHA2_256`](hash::SHA2_256) struct to calculate SHA-2 256 digest.
//!
//! ```rust
//! # use std::fs::File;
//! # use std::path::Path;
//! # use chksum::Result;
//! use chksum::chksum;
//! use chksum::hash::SHA2_256;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let digest = chksum::<SHA2_256, _>(path)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "10a0933e11c86746636370a913ba9be34bc2ac2871585cb0e573c354e7116772"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ### SHA-2 384
//!
//! Use [`SHA2_384`](hash::SHA2_384) struct to calculate SHA-2 384 digest.
//!
//! ```rust
//! # use std::fs::File;
//! # use std::path::Path;
//! # use chksum::Result;
//! use chksum::chksum;
//! use chksum::hash::SHA2_384;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let digest = chksum::<SHA2_384, _>(path)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "7d44140596271726d0e57b3f97615d615d771d48b9b62a0b9da053e68e11992fa9166795671999d036a1e2c17a60414e"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ### SHA-2 512
//!
//! Use [`SHA2_512`](hash::SHA2_512) struct to calculate SHA-2 512 digest.
//!
//! ```rust
//! # use std::fs::File;
//! # use std::path::Path;
//! # use chksum::Result;
//! use chksum::chksum;
//! use chksum::hash::SHA2_512;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let digest = chksum::<SHA2_512, _>(path)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "6bf5a2ab9e922a5723a937acd2b8afa685d20c1f9f16d435fce8c7fd41c3425bbed990bb0a16124f2b62147cd4342496769b789e7186a0e2d85dafc1e5dc626b"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! # Feature flags
//!
//! ## Algorithms
//!
//! * `md5`: Enables MD5 hash algorithm.
//! * `sha1`: Enables SHA-1 hash algorithm.
//! * `sha2`: Enables SHA-2 hash family algorithms.
//!   * `sha2-224`: Enables only SHA-2 224 hash algorithm.
//!   * `sha2-256`: Enables only SHA-2 256 hash algorithm.
//!   * `sha2-384`: Enables only SHA-2 384 hash algorithm.
//!   * `sha2-512`: Enables only SHA-2 512 hash algorithm.
//!
//! By default all of them are enabled.
//!
//! ## Compilation
//!
//! * `inline`: Adds `#[inline]` attribute to some methods on release build.
//!
//! By default all of them are enabled.
//!
//! # License
//!
//! MIT

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![forbid(unsafe_code)]

pub(crate) mod directory;
pub(crate) mod error;
pub(crate) mod path;
pub(crate) mod read;

use std::fs::{DirEntry, File, ReadDir};
use std::io::StdinLock;
use std::path::{Path, PathBuf};
use std::result;

pub use chksum_hash as hash;
use is_terminal::IsTerminal;

use crate::directory::{DirEntryChksumer, ReadDirChksumer};
pub use crate::error::{Error, Result};
use crate::hash::Update;
use crate::path::PathChksumer;
use crate::read::ReadChksumer;

/// A trait for objects which are able to be checksumable.
pub trait Chksum<T>: Update {
    /// The type of the returned error.
    type Error;

    /// Calculates checksum of given input.
    ///
    /// Check [`chksum`] function for more details.
    #[cfg_attr(all(release, feature = "inline"), inline)]
    fn chksum(data: T) -> result::Result<Self::Digest, Self::Error> {
        let args = Args::default();
        Self::chksum_with(data, &args)
    }

    #[doc(hidden)] // TODO: create documentation
    fn chksum_with(data: T, args: &Args) -> result::Result<Self::Digest, Self::Error>;
}

impl<T> Chksum<DirEntry> for T
where
    T: Default + Update,
{
    type Error = Error;

    #[cfg_attr(all(release, feature = "inline"), inline)]
    fn chksum_with(data: DirEntry, args: &Args) -> result::Result<Self::Digest, Self::Error> {
        let hash = Self::default();
        let DirEntryChksumer { hash, .. } = DirEntryChksumer { hash, args }.update(data)?;
        let digest = hash.digest();
        Ok(digest)
    }
}

impl<T> Chksum<File> for T
where
    T: Default + Update,
{
    type Error = Error;

    #[cfg_attr(all(release, feature = "inline"), inline)]
    fn chksum_with(data: File, args: &Args) -> result::Result<Self::Digest, Self::Error> {
        Self::chksum_with(&data, args)
    }
}

impl<T> Chksum<&File> for T
where
    T: Default + Update,
{
    type Error = Error;

    #[cfg_attr(all(release, feature = "inline"), inline)]
    fn chksum_with(data: &File, args: &Args) -> result::Result<Self::Digest, Self::Error> {
        let hash = Self::default();
        let ReadChksumer { hash, .. } = ReadChksumer { hash, args }.update(data)?;
        let digest = hash.digest();
        Ok(digest)
    }
}

impl<T> Chksum<&Path> for T
where
    T: Default + Update,
{
    type Error = Error;

    #[cfg_attr(all(release, feature = "inline"), inline)]
    fn chksum_with(data: &Path, args: &Args) -> result::Result<Self::Digest, Self::Error> {
        let hash = Self::default();
        let PathChksumer { hash, .. } = PathChksumer { hash, args }.update(data)?;
        let digest = hash.digest();
        Ok(digest)
    }
}

impl<T> Chksum<PathBuf> for T
where
    T: Default + Update,
{
    type Error = Error;

    #[cfg_attr(all(release, feature = "inline"), inline)]
    fn chksum_with(data: PathBuf, args: &Args) -> result::Result<Self::Digest, Self::Error> {
        Self::chksum_with(&data, args)
    }
}

impl<T> Chksum<&PathBuf> for T
where
    T: Default + Update,
{
    type Error = Error;

    #[cfg_attr(all(release, feature = "inline"), inline)]
    fn chksum_with(data: &PathBuf, args: &Args) -> result::Result<Self::Digest, Self::Error> {
        Self::chksum_with(data.as_path(), args)
    }
}

impl<T> Chksum<ReadDir> for T
where
    T: Default + Update,
{
    type Error = Error;

    #[cfg_attr(all(release, feature = "inline"), inline)]
    fn chksum_with(data: ReadDir, args: &Args) -> result::Result<Self::Digest, Self::Error> {
        let hash = Self::default();
        let ReadDirChksumer { hash, .. } = ReadDirChksumer { hash, args }.update(data)?;
        let digest = hash.digest();
        Ok(digest)
    }
}

impl<'a, T> Chksum<StdinLock<'a>> for T
where
    T: Default + Update,
{
    type Error = Error;

    #[cfg_attr(all(release, feature = "inline"), inline)]
    fn chksum_with(data: StdinLock<'a>, args: &Args) -> result::Result<Self::Digest, Self::Error> {
        if data.is_terminal() {
            let error = Error::IsTerminal;
            return Err(error);
        }
        let hash = Self::default();
        let ReadChksumer { hash, .. } = ReadChksumer { hash, args }.update(data)?;
        let digest = hash.digest();
        Ok(digest)
    }
}

#[doc(hidden)] // TODO: create documentation
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArgsBuilder {
    pub chunk_size: Option<usize>,
}

impl ArgsBuilder {
    #[cfg_attr(all(release, feature = "inline"), inline)]
    #[must_use]
    pub const fn new() -> Self {
        let chunk_size = None;
        Self { chunk_size }
    }

    #[cfg_attr(all(release, feature = "inline"), inline)]
    #[must_use]
    pub const fn chunk_size(mut self, chunk_size: usize) -> Self {
        self.chunk_size = Some(chunk_size);
        self
    }

    #[cfg_attr(all(release, feature = "inline"), inline)]
    #[must_use]
    pub const fn build(self) -> Args {
        let Self { chunk_size } = self;
        Args { chunk_size }
    }
}

impl Default for ArgsBuilder {
    #[cfg_attr(all(release, feature = "inline"), inline)]
    fn default() -> Self {
        Self::new()
    }
}

#[doc(hidden)] // TODO: create documentation
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Args {
    pub chunk_size: Option<usize>,
}

impl Args {
    #[must_use]
    pub const fn new() -> Self {
        let chunk_size = None;
        Self { chunk_size }
    }
}

impl Default for Args {
    #[cfg_attr(all(release, feature = "inline"), inline)]
    fn default() -> Self {
        Self::new()
    }
}

/// Internal trait to implement different checksumers.
pub(crate) trait Chksumer<T>
where
    Self: Sized,
{
    /// Error type returned when something wrong happened.
    type Error;

    /// Update checksumer with incoming data.
    fn update(self, data: T) -> result::Result<Self, Self::Error>;
}

/// Calculates checksum of given input.
///
/// # Examples
///
/// Choose preferred hash algorithm.
///
/// ```rust
/// # use std::fs::File;
/// # use std::path::Path;
/// # use chksum::Result;
/// use chksum::chksum;
/// use chksum::hash::{MD5, SHA1, SHA2_224};
///
/// # fn wrapper_md5(path: &Path) -> Result<()> {
/// let file = File::open(path)?;
/// let digest = chksum::<MD5, _>(file)?;
/// assert_eq!(
///     digest.to_hex_lowercase(),
///     "91de52cc35ec212a8f406ce89ca28a95"
/// );
/// # Ok(())
/// # }
///
/// // or
///
/// # fn wrapper_sha1(path: &Path) -> Result<()> {
/// let file = File::open(path)?;
/// let digest = chksum::<SHA1, _>(file)?;
/// assert_eq!(
///     digest.to_hex_lowercase(),
///     "399445db3210104118c24061d73900e3d6f3af53"
/// );
/// # Ok(())
/// # }
///
/// // or
///
/// # fn wrapper_sha2_224(path: &Path) -> Result<()> {
/// let file = File::open(path)?;
/// let digest = chksum::<SHA2_224, _>(file)?;
/// assert_eq!(
///     digest.to_hex_lowercase(),
///     "15b4f8536af87424871a8e89d1193a3bd759b2306ea0bc5007f2105b"
/// );
/// # Ok(())
/// # }
/// ```
///
/// You can use different types as an input.
///
/// ```rust
/// # use std::fs::{read_dir, File};
/// # use std::path::{Path, PathBuf};
/// # use chksum::Result;
/// use chksum::chksum;
/// use chksum::hash::{SHA2_224, SHA2_256, SHA2_384};
///
/// # fn wrapper_file(path: &Path) -> Result<()> {
/// // use File as an input
/// let file = File::open(path)?;
/// let digest = chksum::<SHA2_224, _>(file)?;
/// assert_eq!(
///     digest.to_hex_lowercase(),
///     "a657747d15c24a9998186a5798284f1ee2621c4591f766709b4e616d"
/// );
/// # Ok(())
/// # }
///
/// // or
///
/// # fn wrapper_read_dir(path: &Path) -> Result<()> {
/// // use ReadDir as an input
/// let dir = read_dir(path)?;
/// let digest = chksum::<SHA2_256, _>(dir)?;
/// assert_eq!(
///     digest.to_hex_lowercase(),
///     "6c671fec2a733e1275c8e8cb4eb025b13d433cde3d31f6d61e7a216a6d853cb0"
/// );
/// # Ok(())
/// # }
///
/// // or
///
/// # fn wrapper_path(path: &Path) -> Result<()> {
/// // use PathBuf as an input
/// let path = PathBuf::from(path);
/// let digest = chksum::<SHA2_384, _>(path)?;
/// assert_eq!(
///     digest.to_hex_lowercase(),
///     "701e7356774cb7d22d89a0843250c555bf5318f2546f2b64e6a62d22b2ce9a8acda618109eb95683605b97c37fc9ae0e"
/// );
/// # Ok(())
/// # }
/// ```
#[cfg_attr(all(release, feature = "inline"), inline)]
pub fn chksum<T, U>(data: U) -> result::Result<T::Digest, T::Error>
where
    T: Chksum<U>,
{
    T::chksum(data)
}

#[doc(hidden)] // TODO: create documentation
#[cfg_attr(all(release, feature = "inline"), inline)]
pub fn chksum_with<T, U>(data: U, args: &Args) -> result::Result<T::Digest, T::Error>
where
    T: Chksum<U>,
{
    T::chksum_with(data, args)
}
