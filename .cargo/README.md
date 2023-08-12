# chksum

[![GitHub](https://img.shields.io/badge/github-ferric--bytes%2Fchksum-24292e?style=flat-square&logo=github "GitHub")](https://github.com/ferric-bytes/chksum)
[![docs.rs](https://img.shields.io/docsrs/chksum?style=flat-square&logo=docsdotrs "docs.rs")](https://docs.rs/chksum)
[![Coverage](https://img.shields.io/codecov/c/gh/ferric-bytes/chksum?style=flat-square&logo=codecov "Coverage")](https://app.codecov.io/gh/ferric-bytes/chksum)
[![MSRV](https://img.shields.io/badge/MSRV-1.65.0-informational?style=flat-square "MSRV")](https://github.com/ferric-bytes/chksum/blob/master/Cargo.toml)
[![deps.rs](https://deps.rs/crate/chksum/0.2.0/status.svg?style=flat-square "deps.rs")](https://deps.rs/crate/chksum/0.2.0)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg?style=flat-square "unsafe forbidden")](https://github.com/rust-secure-code/safety-dance)
[![LICENSE](https://img.shields.io/github/license/ferric-bytes/chksum?style=flat-square "LICENSE")](https://github.com/ferric-bytes/chksum/blob/master/LICENSE)

High-level interface for easy calculation of checksum digest for files, directories, stdin and more.

## Features

- Written in pure Rust
- Easy to use interface
- No unsafe code
- Configurable via Cargo features

## Setup

Add the following entry to the `dependencies` section of your `Cargo.toml` file:

```toml
[dependencies]
# ...
chksum = "0.2.0"
```

Alternatively, you can use the [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand:

```shell
cargo add chksum
```

## Usage

Use `chksum` function and `File` as an input.

```rust
use std::fs::File;

use chksum::chksum;
use chksum::hash::SHA2_224;

let file = File::open(path)?;
let digest = chksum::<SHA2_224, _>(file)?;
assert_eq!(
    digest.to_hex_lowercase(),
    "a39b86d838273f5ff4879c26f85e3cb333bb44d73b24f275bad1a6c6"
);
```

Alternatively use `ReadDir` as an input.

```rust
use std::fs::read_dir;

use chksum::chksum;
use chksum::hash::SHA2_256;

let dir = read_dir(path)?;
let digest = chksum::<SHA2_256, _>(dir)?;
assert_eq!(
    digest.to_hex_lowercase(),
    "5c3bfbc8614adc72d3ec0e9b15a1fd1c55cee63e34af5a4ff058eb2eef7d8482"
);
```

For more usage examples, refer to the documentation available at [docs.rs](https://docs.rs/chksum).

## Low-level interface

Check [`chksum-hash`](https://crates.io/crates/chksum-hash) for low-level interface.

## License

MIT
