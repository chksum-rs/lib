# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1] - 2023-08-17

### Added

- Added `impl Chksum` for `Stdin`.

### Fixed

- Fixed missing keywords and categories in `Cargo.toml`.
- Fixed missing `Error::IsTerminal` error message.
- Fixed file handling (error is returned when file is terminal).
- Fixed doc description for `Chksum` trait.

### Changed

- Changed `cargo tarpaulin` command to use `--engine llvm` in GitHub Actions.
- Changed `impl Chksum` for `PathBuf` - call method and DRY.
- Changed `impl Chksum` for `File` - call method and DRY.

### Removed

- Removed build script as well as `inline` feature.

## [0.2.0] - 2023-08-12

### Added

- Initial release.

[0.2.1]: https://github.com/ferric-bytes/chksum/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/ferric-bytes/chksum/releases/tag/v0.2.0
