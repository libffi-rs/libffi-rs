# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog] and this project adheres to
[Semantic Versioning].

[Keep a Changelog]: http://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: http://semver.org/spec/v2.0.0.html

## [Unreleased]

- Update to 2021 edition: https://github.com/tov/libffi-rs/pull/121
- Set rust-version to 1.78: https://github.com/tov/libffi-rs/pull/120
- Enable long double for all targets: https://github.com/tov/libffi-rs/pull/130
- Fix build on mingw: https://github.com/tov/libffi-rs/pull/118
- Fix Sparcv9: https://github.com/tov/libffi-rs/pull/129
- Marked deprecated libffi functions as deprecated: https://github.com/tov/libffi-rs/pull/128
- Updated trampoline sizes for x86_win32, x86, aarch64 (non-Apple): https://github.com/tov/libffi-rs/pull/127
- Added correct default ABI for ARM with hardware floats: https://github.com/tov/libffi-rs/pull/127
- Added Windows ABI to aarch64 and set it as the default ABI for Windows: https://github.com/tov/libffi-rs/pull/127

## [3.2.0] - 2025-04-09

- Fix out-of-bound reads on x84-64 for small integers: https://github.com/tov/libffi-rs/commit/fe3115a7627f77445530f072541ad31502fc6f03

## [3.1.0] - 2025-04-08

- Fix several cross-compilation warnings: https://github.com/tov/libffi-rs/commit/d8705c3836e24bd024d6881d0cfd73d0ea9749af

## [3.0.0] - 2025-04-08

- Add constants for the MIPS family: https://github.com/tov/libffi-rs/commit/6e5190640b5f54646fd1fa6eeaae8a238d43c497
- Don't preserve libffi mode and ownership on build: https://github.com/tov/libffi-rs/commit/ef98286d3cd01ddb29f441f0794fadddd420f5a3
- Correct INCLUDE for msvc builds: https://github.com/tov/libffi-rs/commit/d930ae9b50620b830465b96b38eab9bb73c35b33
- Fix build for riscv64gc-unknown-linux-musl: https://github.com/tov/libffi-rs/commit/ed5f28d3d16e9a8177b7f79115f76f96ef150b09
- Fix build for powerpc64-unknown-linux-musl: https://github.com/tov/libffi-rs/commit/f9fb3acd1a19a1a61b9164f0917498afea1d7b19
- Add support for the sparcv9 architecture: https://github.com/tov/libffi-rs/commit/3bd44239750b50b7bc2f5e9586346dec6d13d0fe
- Update libffi to v3.4.7: https://github.com/tov/libffi-rs/commit/80756374975bde22871363c4dd88693e9a00e66f

## [2.3.0] - 2023-04-26

- Add support for loongarch64: https://github.com/tov/libffi-rs/pull/75
- Fix the build process to always use the local "configure" script: https://github.com/tov/libffi-rs/pull/76

## [2.2.1] - 2023-04-02

- Fix building with clang 16 and newer: https://github.com/tov/libffi-rs/pull/74

## [2.2.0] - 2023-03-28

- Add support for aarch64-apple-ios-sim: https://github.com/tov/libffi-rs/pull/64
- Support aarch64 MSVC target: https://github.com/tov/libffi-rs/pull/65
- Add support for the s390x architecture: https://github.com/tov/libffi-rs/pull/70

## [2.1.0] - 2023-01-02

- Update libffi to 3.4.4: https://github.com/tov/libffi-rs/pull/63

## [2.0.1] - 2022-10-20

- Fix cross-compiling to Illumos: https://github.com/tov/libffi-rs/pull/59

## [2.0.0] - 2022-03-07

- Improve cross-compilation support: https://github.com/tov/libffi-rs/pull/53
- Rust 1.48 or newer is now required

## [1.3.2] - 2022-02-05

### Fixed

- Fix linker error with C long double on aarch64

## [1.3.1] - 2022-02-02

### Changed

- Added support for RISC-V

## [1.3.0] - 2021-10-26

### Changed

- The source code of libffi is now included directly, rather than through a
  submodule. This removes the dependency on autotools. See
  https://github.com/tov/libffi-rs/pull/33 for more information.

## [1.2.0] - 2021-10-24

### Changed

- Updated libffi to 3.4.2

## [1.1.3] - 2021-09-10

### Changed

- The dependency on make-cmd has been removed, and we now always use the `make`
  command to compile libffi

## [1.1.2] - 2021-08-17

### Changed

- Remove outdated documentation links

## [1.1.1] - 2021-05-06

- Fix assert! deprecation in the build script

## [1.1.0] - 2020-11-11

### Changed
- Improved portability by adding support for architectures such as PowerPC and
  i686, courtesy of Andrew Gaspar
- Added support for ARMv7, courtesy of Tim Fish

## [1.0.0] - 2020-10-25

### Changed
- Replace bindgen dependency with manually maintained libffi bindings. This
  removes the need for installing clang, and reduces the amount of build-time
  Rust dependencies.
- Unset DESTDIR when building libffi.

## [0.9.1] - 2019-12-29

### Added
- Windows support (GNU or MSVC toolchain).

### Changed
- Updated Rust edition to 2018.

## [0.9.0] - 2019-12-07

### Changed
- Updated version of automatically-built C libffi to 3.3.
- No longer builds C libffi documentation (and thus we no longer depend on
  Texinfo.

## [0.8.0] - 2019-10-19

### Changed
- Updated version of `bindgen` build dependency to `^0.51` from
  `0.49`. (As a consequence, we now require rustc 1.32.0 or later.)

## [0.7.0] - 2019-05-12

### Fixed
- Yanked previous version (0.6.4), because updating the
  `bindgen` dependency was not semver-compatible.

## [0.6.4] - 2019-04-10

### Changed
- Updated version of `bindgen` build dependency to `^0.49` from
  `0.31.3`. (As a consequence, we now require rustc 1.30.0 or later.)

## [0.6.3] - 2018-10-29

### Added
- Windows support via MSYS or MinGW.

## [0.6.2] - 2018-08-21

### Added
- Feature `system` probes for system libffi instead of downloading and
  bulding our own.

## [0.6.1] - 2018-05-30

### Added
- Setting `doc(html_root_url)` for inter-crate docs linking.
- Testing on Rust 1.20.0 now, as oldest supported version.
- Better message when bindgen fails.

### [0.6.0] - 2017-11-13

### Changed
- Upgraded to `bindgen` 0.31.3.

### [0.5.4] - 2017-11-12

### Changed
- Calling `bindgen` with `blacklist_type` rather than `hide_type`, as the
latter is deprecated. (Thanks, fitzgen.)

### [0.5.3] - 2017-07-07

### Added
- `lib64/` now in library search path.
- Build instructions now mention C libffi and texinfo.

### [0.5.2] - 2017-04-14

### Fixed
- Avoid some unnecessary C libffi rebuilds. (Thanks, ngkz.)
- Avoids link error on Arch Linux by building C libffi `--withpic`.

### Changed
- Links against a self-build static C libffi rather than dynamic. (Thanks,
  ngkz.)

### [0.5.0] - 2017-03-02

### Removed
- No longer passing `--disable-docs` to `configure` for C libffi.

## [0.4.7] - 2017-03-01

### Changed
- Hiding `max_align_t` struct in `stddef.h` from bindgen, because it was
  confusing it.
- Upgraded bindgen (0.22). (Thanks, cholcombe973.)

### Added

- Bindgen now generates default impls. (Thanks, cholcombe973.)

## [0.4.6] - 2016-08-29

### Changed
- Upgraded bindgen (0.18).

## [0.4.4] - 2016-06-21

### Changed
- Builds C libffi from a Git submodule. (Thanks, murarth.)

## [0.4.3] - 2016-06-21

### Changed
- Builds dynamic C libffi.

## [0.4.2] - 2016-06-20

### Changed
- Fetching C libffi from a cached copy on my website, because fetching it
  from ftp is unreliable.

## [0.4.1] - 2016-06-20

### Changed
- Using `-lffi` instead of `-llibffi`.

## [0.4.0] - 2016-06-20

### Added
- Fetches and builds its own C libffi now.

## [0.3.4] - 2016-06-17

### Changed
- Updated `clang-sys` version.

## [0.3.3] - 2016-06-14

### Fixed
- Crate name in instructions.

## [0.3.2] - 2016-06-14

### Added
- Better error messages from `build.rs`.
- Clarified dependencies in docs.

## [0.3.0] - 2016-06-14

Split from `libffi` crate.

