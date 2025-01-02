//! This crate provides an implementation of various hash functions with a straightforward interface for computing digests of bytes, files, directories, and more.
//!
//! For a low-level interface, you can explore the [`chksum_hash`] crate.
//!
//! # Setup
//!
//! To use this crate, add the following entry to your `Cargo.toml` file in the `dependencies` section:
//!
//! ```toml
//! [dependencies]
//! chksum = "0.3.0"
//! ```
//!
//! Alternatively, you can use the [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand:
//!
//! ```sh
//! cargo add chksum
//! ```     
//!
//! # Usage
//!
//! Use the [`chksum`] function with the desired algorithm to calculate digest of file, directory and so on.
//!
//! ```rust
//! # use std::path::Path;
//! use std::fs::File;
//!
//! # use chksum::Result;
//! use chksum::{chksum, SHA2_256};
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let file = File::open(path)?;
//! let digest = chksum::<SHA2_256>(file)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "44752f37272e944fd2c913a35342eaccdd1aaf189bae50676b301ab213fc5061"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! Alternatively, use the `chksum` function directly from the chosen hash module.
//!
//! ```rust
//! # use std::path::Path;
//! use std::fs::File;
//!
//! # use chksum::Result;
//! use chksum::sha2_256;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let file = File::open(path)?;
//! let digest = sha2_256::chksum(file)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "44752f37272e944fd2c913a35342eaccdd1aaf189bae50676b301ab213fc5061"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! # Input Types
//!
//! ## Bytes
//!
//! ### Array
//!
//! ```rust
//! # use chksum::Result;
//! use chksum::sha2_256;
//!
//! # fn wrapper() -> Result<()> {
//! let data = [0, 1, 2, 3];
//! let digest = sha2_256::chksum(data)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "44752f37272e944fd2c913a35342eaccdd1aaf189bae50676b301ab213fc5061"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ### Vec
//!
//! ```rust
//! # use chksum::Result;
//! use chksum::sha2_256;
//!
//! # fn wrapper() -> Result<()> {
//! let data = vec![0, 1, 2, 3];
//! let digest = sha2_256::chksum(data)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "44752f37272e944fd2c913a35342eaccdd1aaf189bae50676b301ab213fc5061"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ### Slice
//!
//! ```rust
//! # use chksum::Result;
//! use chksum::sha2_256;
//!
//! # fn wrapper() -> Result<()> {
//! let data = &[0, 1, 2, 3];
//! let digest = sha2_256::chksum(data)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "44752f37272e944fd2c913a35342eaccdd1aaf189bae50676b301ab213fc5061"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## Strings
//!
//! ### str
//!
//! ```rust
//! # use chksum::Result;
//! use chksum::sha2_256;
//!
//! # fn wrapper() -> Result<()> {
//! let data = "&str";
//! let digest = sha2_256::chksum(data)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "44752f37272e944fd2c913a35342eaccdd1aaf189bae50676b301ab213fc5061"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ### String
//!
//! ```rust
//! # use chksum::Result;
//! use chksum::sha2_256;
//!
//! # fn wrapper() -> Result<()> {
//! let data = String::from("String");
//! let digest = sha2_256::chksum(data)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "44752f37272e944fd2c913a35342eaccdd1aaf189bae50676b301ab213fc5061"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## File
//!
//! ```rust
//! # use std::path::Path;
//! use std::fs::File;
//!
//! # use chksum::Result;
//! use chksum::sha2_256;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let file = File::open(path)?;
//! let digest = sha2_256::chksum(file)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "44752f37272e944fd2c913a35342eaccdd1aaf189bae50676b301ab213fc5061"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## Directory
//!
//! ```rust
//! # use std::path::Path;
//! use std::fs::read_dir;
//!
//! # use chksum::Result;
//! use chksum::sha2_256;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let readdir = read_dir(path)?;
//! let digest = sha2_256::chksum(readdir)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "44752f37272e944fd2c913a35342eaccdd1aaf189bae50676b301ab213fc5061"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## Path
//!
//! ```rust
//! # use std::path::Path;
//! use std::path::PathBuf;
//!
//! # use chksum::Result;
//! use chksum::sha2_256;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let path = PathBuf::from(path);
//! let digest = sha2_256::chksum(path)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "44752f37272e944fd2c913a35342eaccdd1aaf189bae50676b301ab213fc5061"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## Standard Input
//!
//! ```rust
//! use std::io::stdin;
//!
//! # use chksum::Result;
//! use chksum::sha2_256;
//!
//! # fn wrapper() -> Result<()> {
//! let stdin = stdin();
//! let digest = sha2_256::chksum(stdin)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "44752f37272e944fd2c913a35342eaccdd1aaf189bae50676b301ab213fc5061"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! # Algorithms
//!
//! ## MD5
//!
//! ```rust
//! # use std::path::Path;
//! use std::fs::File;
//!
//! # use chksum::Result;
//! use chksum::md5;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let file = File::open(path)?;
//! let digest = md5::chksum(file)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "5c71dbb287630d65ca93764c34d9aa0d"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## SHA-1
//!
//! ```rust
//! # use std::path::Path;
//! use std::fs::File;
//!
//! # use chksum::Result;
//! use chksum::sha1;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let file = File::open(path)?;
//! let digest = sha1::chksum(file)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "9fc42adac31303d68b444e6129f13f6093a0e045"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## SHA-2 224
//!
//! ```rust
//! # use std::path::Path;
//! use std::fs::File;
//!
//! # use chksum::Result;
//! use chksum::sha2_224;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let file = File::open(path)?;
//! let digest = sha2_224::chksum(file)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "90382cbfda2656313ad61fd74b32ddfa4bcc118f660bd4fba9228ced"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## SHA-2 256
//!
//! ```rust
//! # use std::path::Path;
//! use std::fs::File;
//!
//! # use chksum::Result;
//! use chksum::sha2_256;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let file = File::open(path)?;
//! let digest = sha2_256::chksum(file)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "44752f37272e944fd2c913a35342eaccdd1aaf189bae50676b301ab213fc5061"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## SHA-2 384
//!
//! ```rust
//! # use std::path::Path;
//! use std::fs::File;
//!
//! # use chksum::Result;
//! use chksum::sha2_384;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let file = File::open(path)?;
//! let digest = sha2_384::chksum(file)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "12ecdfd463a85a301b7c29a43bf4b19cdfc6e5e86a5f40396aa6ae3368a7e5b0ed31f3bef2eb3071577ba610b4ed1cb8"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## SHA-2 512
//!
//! ```rust
//! # use std::path::Path;
//! use std::fs::File;
//!
//! # use chksum::Result;
//! use chksum::sha2_512;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let file = File::open(path)?;
//! let digest = sha2_512::chksum(file)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "ed59c5759a9ece516cec0c0623142d0e9fe70a27d750eee7fd38f4550d50addd873d0fa1a51fc823c1e3d5cada203f4a05d8325caacb7d3e0727a701f3f07e5f"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! # Features
//!
//! ## Algorithms
//!
//! Cargo features are utilized to enable or disable specific hash algorithms.
//!
//! * `md5` enables MD5, accessible via the [`md5`] module.
//! * `sha1` enables SHA-1, accessible via the [`sha1`] module.
//! * `sha2-224` enables SHA-2 224, accessible via the [`sha2_224`] module.
//! * `sha2-256` enables SHA-2 256, accessible via the [`sha2_256`] module.
//! * `sha2-384` enables SHA-2 384, accessible via the [`sha2_384`] module.
//! * `sha2-512` enables SHA-2 512, accessible via the [`sha2_512`] module.
//!
//! By default, all of these features are enabled.
//!
//! To customize your setup, disable the default features and enable only those that you need in your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! chksum = { version = "0.3.0", default-features = false, features = ["sha1", "sha2-256", "sha2-512"] }
//! ```
//!
//! Alternatively, you can use the [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand:
//!
//! ```shell
//! cargo add chksum --no-default-features --features sha1,sha2-256,sha2-512
//! ```
//!
//! ## Extra Options
//!
//! Cargo features are also utilized to enable extra options.
//!
//! * `reader` enables the `reader` module with the `Reader` struct within each variant module.
//! * `writer` enables the `writer` module with the `Writer` struct within each variant module.
//!
//! By default, neither of these features is enabled.
//!
//! To customize your setup, disable the default features and enable only those that you need in your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! chksum = { version = "0.3.0", features = ["reader", "writer"] }
//! ```
//!
//! Alternatively, you can use the [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand:
//!
//! ```shell
//! cargo add chksum --features reader,writer
//! ```
//!
//! # License
//!
//! This crate is licensed under the MIT License.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![forbid(unsafe_code)]

#[cfg(feature = "async-runtime-tokio")]
#[doc(no_inline)]
pub use chksum_core::{async_chksum, AsyncChksumable};
#[doc(no_inline)]
pub use chksum_core::{chksum, hash, Chksumable, Digest, Error, Hash, Hashable, Result};
#[cfg(docsrs)]
use chksum_hash;
#[cfg(feature = "md5")]
#[doc(no_inline)]
pub use chksum_md5 as md5;
#[cfg(feature = "md5")]
#[doc(no_inline)]
pub use chksum_md5::MD5;
#[cfg(feature = "reader")]
#[doc(no_inline)]
pub use chksum_reader as reader;
#[cfg(feature = "reader")]
#[doc(no_inline)]
pub use chksum_reader::Reader;
#[cfg(feature = "sha1")]
#[doc(no_inline)]
pub use chksum_sha1 as sha1;
#[cfg(feature = "sha1")]
#[doc(no_inline)]
pub use chksum_sha1::SHA1;
#[cfg(any(
    feature = "sha2-224",
    feature = "sha2-256",
    feature = "sha2-384",
    feature = "sha2-512",
))]
#[doc(no_inline)]
pub use chksum_sha2 as sha2;
#[cfg(feature = "sha2-224")]
#[doc(no_inline)]
pub use chksum_sha2::sha2_224;
#[cfg(feature = "sha2-256")]
#[doc(no_inline)]
pub use chksum_sha2::sha2_256;
#[cfg(feature = "sha2-384")]
#[doc(no_inline)]
pub use chksum_sha2::sha2_384;
#[cfg(feature = "sha2-512")]
#[doc(no_inline)]
pub use chksum_sha2::sha2_512;
#[cfg(feature = "sha2-224")]
#[doc(no_inline)]
pub use chksum_sha2::SHA2_224;
#[cfg(feature = "sha2-256")]
#[doc(no_inline)]
pub use chksum_sha2::SHA2_256;
#[cfg(feature = "sha2-384")]
#[doc(no_inline)]
pub use chksum_sha2::SHA2_384;
#[cfg(feature = "sha2-512")]
#[doc(no_inline)]
pub use chksum_sha2::SHA2_512;
#[cfg(feature = "writer")]
#[doc(no_inline)]
pub use chksum_writer as writer;
#[cfg(feature = "writer")]
#[doc(no_inline)]
pub use chksum_writer::Writer;
