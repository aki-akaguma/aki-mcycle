# Changelog: aki-mcycle

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Code review report `docs/reviews/2026-06-01_code_review.3.md`.

### Changed
- Use `Vec::retain` for more idiomatic and efficient collection cleanup in `src/run.rs`.
- Improve readability of regex capture group selection logic in `src/run.rs`.

## [0.2.1] - 2026-05-19

### Added
- Code review report `docs/review.2.md`.
- Code review report `docs/review.1.md`.
- Documentation for stateful coloring feature in `README.md`, `src/lib.rs`, and `specs/0.requirements.md`.
- Performance section in `README.md` and `src/lib.rs`.

### Changed
- Update crates: `flood-tide` (0.2.14) and `flood-tide-gen` (0.2.2).
- Update crates: `runnel` (0.4.2) and `regex` (1.12).
- Set minimum supported Rust version (MSRV) to 1.68.0.
- Update `specs/2.design.md` to reflect the use of `flood-tide` instead of `clap`.
- Update MSRV in `README.md` and `src/lib.rs`.
- Use sparse representation for coloring marks in `src/run.rs` to improve memory efficiency on extremely long lines.

### Fixed
- Spelling of 'magenta' and environment variable `AKI_MCYCLE_COLOR_SEQ_MAGENTA_ST`.
- Uninlined format arguments (`clippy::uninlined-format-args`).
- Needless borrow (`clippy::needless_borrow`).

### Removed
- Crate `memx-cdy`.

## [0.2.0] - 2025-09-15

### Added
- Specifications in `specs/`.
- Additional tests.
- Function `execute_with_env()`.

### Changed
- `IntoIterator` compatibility for arguments in `execute()`.
- Update crate `runnel` (0.4.0).
- Update crate `rust-version-info-file` (0.2.0).
- Update crate `regex` (1.11).
- Refactor `src/run.rs`.
- Refactor `src/lib.rs`.

### Fixed
- MSRV in documentation.

### Removed
- Function `execute_env()`.
- `base_dir=` from `-X` options.

## [0.1.29] - 2024-06-19

### Added
- GitHub Actions workflows for Ubuntu, macOS, and Windows.
- Test status badges in `README.tpl`.
- Miri support for tests.
- `tarpaulin` support in `Makefile`.

### Changed
- Rename `config` to `config.toml`.
- Refactor `Makefile`.
- Update dependencies: `flood-tide` (0.2.11) and `flood-tide-gen` (0.1.22).
- Update dependencies: `memx-cdy` (0.1.13) and `runnel` (0.3.19).
- Update dependencies: `exec-target` (0.2.8), `indoc` (2.0.0), and `rust-version-info-file` (0.1.10).
- Increase MSRV from 1.56.0 to 1.65.0.

### Fixed
- License files `LICENSE-APACHE` and `LICENSE-MIT`.
- Clippy lints: `redundant_static_lifetimes`, `needless_borrow`, `bool_assert_comparison`, `uninlined_format_args`, `unused_imports`, `items_after_test_module`, and `derivable_impls`.

### Removed
- `cfg(has_not_matches)`.
- File `COPYING`.

## [0.1.28] - 2023-01-11

### Added
- Badges in `README.tpl`.
- MSRV 1.56.0 in `Cargo.toml`.

### Changed
- Reformat `CHANGELOG.md`.
- Update dependency `anyhow` (1.0.68).
- Update dependencies: `flood-tide` (0.2.8) and `flood-tide-gen` (0.1.19).
- Update dependencies: `memx-cdy` (0.1.10) and `runnel` (0.3.15).
- Update dependency `regex` (1.7.1).

### Fixed
- Implement `Eq` where `PartialEq` is derived (`clippy::derive_partial_eq_without_eq`).
- Clippy lint `bool_to_int_with_if`.
- Clippy lint `uninlined_format_args`.

## [0.1.27] - 2022-06-18

### Changed
- Upgrade to Rust 2021 edition.
- Update dependencies: `flood-tide` (0.2.5), `memx` (0.1.21), `memx-cdy` (0.1.8), `runnel` (0.3.11), `exec-target` (0.2.6), `flood-tide-gen` (0.1.16), `rust-version-info-file` (0.1.6), and `semver` (1.0.10).

## [0.1.26] - 2022-05-22

### Changed
- Update dependencies: `runnel` (0.3.10), `memx` (0.1.20), `anyhow` (1.0.57), `libc` (0.2.126), `regex` (1.5.6), `exec-target` (0.2.5), and `rust-version-info-file` (0.1.5).

## [0.1.25] - 2021-11-15

### Added
- Additional documentation.

### Changed
- Set MSRV to 1.47.0.
- Update dependencies: `flood-tide` (0.2.4), `memx` (0.1.18), `memx-cdy` (0.1.7), `runnel` (0.3.9), `anyhow` (1.0.45), `libc` (0.2.107), `exec-target` (0.2.4), `flood-tide-gen` (0.1.15), and `rust-version-info-file` (0.1.3).

## [0.1.24] - 2021-09-11

### Added
- Dependency `indoc` (1.0.3).

### Changed
- Update dependencies: `anyhow` (1.0.43), `flood-tide-gen` (0.1.14), `flood-tide` (0.2.3), `memx-cdy` (0.1.6), and `runnel` (0.3.8).
- Rewrite `TARGET_EXE_PATH` using `env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")))`.
- Update dependency `exec-target` (0.2.3).

### Fixed
- Clippy lints.

## [0.1.23] - 2021-06-24

### Added
- Fast memory operations via `memx_cdy::memx_init()`.

### Changed
- Rewrite `TARGET_EXE_PATH` using `env!("CARGO_BIN_EXE_aki-mcycle")`.

### Fixed
- Conditional compilation for `debian_build` feature.

## [0.1.22] - 2021-06-03

### Added
- Support for `debian_build` feature.

### Changed
- Update dependencies: `flood-tide` (0.2.2) and `regex` (1.5.4).

### Fixed
- Command option `-X rust-version-info`.

## [0.1.21] - 2021-04-23

### Fixed
- Bug in `build.rs`.

## [0.1.20] - 2021-04-23

### Added
- Command option `-X`.

### Changed
- Update dependencies: `flood-tide-gen` (0.1.12), `flood-tide` (0.2.1), and `regex` (1.4.6).

## [0.1.19] - 2021-04-19

### Changed
- Update dependency `flood-tide-gen` (0.1.10).

## [0.1.18] - 2021-04-07

### Changed
- Update dependencies: `flood-tide` (0.2), `anyhow` (1.0.40), `flood-tide-gen` (0.1.8), and `runnel` (0.3.6).

## [0.1.17] - 2021-03-22

### Added
- Function `execute_env()`.
- Tests.
- Contents to `--help`.

### Changed
- Improve environment handling.
- Update dependency `regex` (1.4.5) to fix stack overflows.

## [0.1.16] - 2021-03-14

### Changed
- Update crate `regex` to fix memory leak.

## [0.1.15] - 2021-03-08

### Added
- Debug tests.

### Changed
- Update crate `runnel`.
- Update crate `rustc_version` (0.3).

## [0.1.14] - 2021-03-08

### Changed
- Update crate `regex` (1.4).
- Update crate `runnel`.
- Rename `xtask/src/cmd.txt` to `xtask/src/aki-mcycle-cmd.txt`.

## [0.1.13] - 2021-03-02

### Added
- Documentation.

### Changed
- Option `-e`, `--regex` to `-e`, `--exp`.
- Environment variable `RUST_CYCLE_COLOR_RED_ST` to `AKI_MCYCLE_COLOR_RED_ST`.
- Update crate `flood-tide-gen`.
- Clean up `src/main.rs` and `build.rs`.

## [0.1.12] - 2021-02-22

### Changed
- Update crates `runnel` and `flood-tide-gen`.

### Fixed
- Missing `flush()` on finish.

## [0.1.11] - 2021-02-17

### Added
- Documentation.

### Changed
- Update crate `runnel`.
- Rename section "AAA-admin" to "AAA-text" in `package.metadata.deb`.

## [0.1.10] - 2021-02-08

### Changed
- Perform initial GitHub release.
- Rename project from `rust-cycle-color` to `aki-mcycle`.

## [0.1.9] - 2021-02-08

### Added
- Subproject `xtask`.
- Stream module.

### Changed
- Import crate `exec-target` from local for testing.
- Switch from `optpa_util_1` to `flood-tide` and `flood-tide-gen`.
- `AppError` to `anyhow::Error`.

## [0.1.8] - 2020-12-29

### Changed
- Update crates.

### Removed
- Crate `optpaerr-1`.

## [0.1.7] - 2020-11-17

### Added
- `README.md`, `COPYING`, `LICENSE-APACHE`, and `LICENSE-MIT` files.

### Changed
- Use `rustc_version` (=0.2.3) as v0.3.0 does not compile on Debian 10 Buster.
- `optpa_util` to `optpa_util_1`.

## [0.1.6] - 2020-08-09

### Added
- Support for `cargo deb`.

### Changed
- Update crates.

## [0.1.5] - 2020-05-10

### Changed
- Upgrade from Rust 2015 to 2018 edition.
- Update crates.

## [0.1.4] - 2020-03-30

### Added
- Support for broken pipes and related tests.

### Changed
- Update crates.

## [0.1.3] - 2019-04-14

### Added
- Support for `std::alloc`.

### Changed
- Update crates.
- Apply miscellaneous updates.

## [0.1.2] - 2018-05-04

### Added
- Support for `cfg(has_global_allocator)`.

### Changed
- Update crates.

## [0.1.1] - 2018-03-22

### Added
- Support for regex capture groups.
- Support for broken pipes.

### Changed
- Update crates.
- Apply miscellaneous updates.

## [0.1.0] - 2017-12-05

### Added
- Initial commit.

[Unreleased]: https://github.com/aki-akaguma/aki-mcycle/compare/v0.2.1..HEAD
[0.2.1]: https://github.com/aki-akaguma/aki-mcycle/compare/v0.2.0..v0.2.1
[0.2.0]: https://github.com/aki-akaguma/aki-mcycle/compare/v0.1.29..v0.2.0
[0.1.29]: https://github.com/aki-akaguma/aki-mcycle/compare/v0.1.28..v0.1.29
[0.1.28]: https://github.com/aki-akaguma/aki-mcycle/compare/v0.1.27..v0.1.28
[0.1.27]: https://github.com/aki-akaguma/aki-mcycle/compare/v0.1.26..v0.1.27
[0.1.26]: https://github.com/aki-akaguma/aki-mcycle/compare/v0.1.25..v0.1.26
[0.1.25]: https://github.com/aki-akaguma/aki-mcycle/compare/v0.1.24..v0.1.25
[0.1.24]: https://github.com/aki-akaguma/aki-mcycle/compare/v0.1.23..v0.1.24
[0.1.23]: https://github.com/aki-akaguma/aki-mcycle/compare/v0.1.22..v0.1.23
[0.1.22]: https://github.com/aki-akaguma/aki-mcycle/compare/v0.1.21..v0.1.22
[0.1.21]: https://github.com/aki-akaguma/aki-mcycle/compare/v0.1.20..v0.1.21
[0.1.20]: https://github.com/aki-akaguma/aki-mcycle/compare/v0.1.19..v0.1.20
[0.1.19]: https://github.com/aki-akaguma/aki-mcycle/compare/v0.1.18..v0.1.19
[0.1.18]: https://github.com/aki-akaguma/aki-mcycle/compare/v0.1.17..v0.1.18
[0.1.17]: https://github.com/aki-akaguma/aki-mcycle/compare/v0.1.16..v0.1.17
[0.1.16]: https://github.com/aki-akaguma/aki-mcycle/compare/v0.1.15..v0.1.16
[0.1.15]: https://github.com/aki-akaguma/aki-mcycle/compare/v0.1.14..v0.1.15
[0.1.14]: https://github.com/aki-akaguma/aki-mcycle/compare/v0.1.13..v0.1.14
[0.1.13]: https://github.com/aki-akaguma/aki-mcycle/compare/v0.1.12..v0.1.13
[0.1.12]: https://github.com/aki-akaguma/aki-mcycle/compare/v0.1.11..v0.1.12
[0.1.11]: https://github.com/aki-akaguma/aki-mcycle/compare/v0.1.10..v0.1.11
[0.1.10]: https://github.com/aki-akaguma/aki-mcycle/releases/tag/v0.1.10
