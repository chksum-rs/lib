# chksum

[![GitHub](https://img.shields.io/badge/github-chksum--rs%2Flib-24292e?style=flat-square&logo=github "GitHub")](https://github.com/chksum-rs/lib)
[![Build](https://img.shields.io/github/actions/workflow/status/chksum-rs/lib/rust.yml?branch=master&style=flat-square&logo=github "Build")](https://github.com/chksum-rs/lib/actions/workflows/rust.yml)
[![docs.rs](https://img.shields.io/docsrs/chksum?style=flat-square&logo=docsdotrs "docs.rs")](https://docs.rs/chksum/)
[![MSRV](https://img.shields.io/badge/MSRV-1.70.0-informational?style=flat-square "MSRV")](https://github.com/chksum-rs/lib/blob/master/Cargo.toml)
[![deps.rs](https://deps.rs/crate/chksum/0.3.0/status.svg?style=flat-square "deps.rs")](https://deps.rs/crate/chksum/0.3.0)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg?style=flat-square "unsafe forbidden")](https://github.com/rust-secure-code/safety-dance)
[![LICENSE](https://img.shields.io/github/license/chksum-rs/lib?style=flat-square "LICENSE")](https://github.com/chksum-rs/lib/blob/master/LICENSE)

An implementation of various hash functions with a straightforward interface for computing digests of bytes, files, directories, and more.

## Setup

To use this crate, add the following entry to your `Cargo.toml` file in the `dependencies` section:

```toml
[dependencies]
chksum = "0.3.0"
```

Alternatively, you can use the [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand:

```shell
cargo add chksum
```

## Usage

Use the `chksum` function with the desired algorithm to calculate digest of file, directory and so on.

```rust
use chksum::{chksum, SHA2_256};

let file = File::open(path)?;
let digest = chksum::<SHA2_256>(file)?;
assert_eq!(
    digest.to_hex_lowercase(),
    "44752f37272e944fd2c913a35342eaccdd1aaf189bae50676b301ab213fc5061"
);
```

Alternatively, use the `chksum` function directly from the chosen hash module.

```rust
use chksum::sha2_256;

let file = File::open(path)?;
let digest = sha2_256::chksum(file)?;
assert_eq!(
    digest.to_hex_lowercase(),
    "44752f37272e944fd2c913a35342eaccdd1aaf189bae50676b301ab213fc5061"
);
```

For more usage examples, refer to the documentation available at [docs.rs](https://docs.rs/chksum/).

## Hash Algorithms

* MD5 - [RFC 1321: The MD5 Message-Digest Algorithm](https://tools.ietf.org/html/rfc1321)
* SHA-1 - [RFC 3174: US Secure Hash Algorithm 1 (SHA1)](https://tools.ietf.org/html/rfc3174)
* SHA-2 family (SHA-224, SHA-256, SHA-386, SHA-512) - [FIPS PUB 180-4: Secure Hash Standard](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf)

## Features

### Algorithms

Cargo features are utilized to enable or disable specific hash algorithms.

* `md5`: Enables MD5 hash algorithm.
* `sha1`: Enables SHA-1 hash algorithm.
* `sha2`: Enables SHA-2 hash family algorithms.
  * `sha2-224`: Enables only SHA-2 224 hash algorithm.
  * `sha2-256`: Enables only SHA-2 256 hash algorithm.
  * `sha2-384`: Enables only SHA-2 384 hash algorithm.
  * `sha2-512`: Enables only SHA-2 512 hash algorithm.

By default, all of them are enabled.

### Extra Options

Cargo features are also utilized to enable extra options.

* `reader` enables the `reader` module with the `Reader` struct within each variant module.
* `writer` enables the `writer` module with the `Writer` struct within each variant module.

By default, neither of these features is enabled.

## License

This crate is licensed under the MIT License.
