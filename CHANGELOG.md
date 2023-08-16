# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- Fixed missing keywords and categories in `Cargo.toml`.

### Changed

- Changed `cargo tarpaulin` command to use `--engine llvm` in GitHub Actions.
- Changed `impl Chksum` for `PathBuf` - call method and DRY.
- Changed `impl Chksum` for `File` - call method and DRY.

## [0.2.0] - 2023-08-12

### Added

- Initial release.

[Unreleased]: https://github.com/ferric-bytes/chksum/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/ferric-bytes/chksum/releases/tag/v0.2.0
