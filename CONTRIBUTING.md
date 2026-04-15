<!--
SPDX-FileCopyrightText: 2026 Chen Linxuan <me@black-desk.cn>

SPDX-License-Identifier: MIT
-->

# Contributing to vcpkg-manifest-rs

en | [zh_CN](CONTRIBUTING.zh_CN.md)

Thank you for your interest in contributing! This document covers the project
architecture, design decisions, and development workflow to help you get
started.

## Project Overview

vcpkg-manifest-rs provides a `build.rs` library (`vcpkg-manifest` crate) for
finding C/C++ dependencies via vcpkg or system-wide search. The actual CMake
package discovery is delegated to
[cmake-package-rs](https://github.com/danvratil/cmake-package-rs).

## Architecture

```
vcpkg-manifest-rs
├── src/
│   └── lib.rs            # vcpkg_manifest library + FindPackageBuilder
├── tests/
│   ├── integration_test.rs  # Integration tests
│   └── fixtures/
│       └── hello-fmt/    # Test fixture (Rust + C++ via vcpkg fmt library)
└── .github/workflows/    # CI and CD pipelines
```

### Dependency Graph

```
build.rs of downstream crate
    └── vcpkg_manifest (lib.rs)
            └── cmake-package
```

## Design Details

### vcpkg Detection Strategy

The library detects whether vcpkg is in use by checking for a
`vcpkg_installed/` directory in the project root (i.e. `CARGO_MANIFEST_DIR`).
This directory is created by `vcpkg install` in manifest mode.

When detected, `find_package()` configures CMake to use vcpkg's toolchain
file; otherwise it falls back to system-wide CMake search. This means the
library works transparently with or without vcpkg.

### VCPKG_ROOT Discovery

`find_vcpkg_root()` locates the vcpkg installation in this order:

1. `VCPKG_ROOT` environment variable
2. Ancestor directory search (from `CARGO_MANIFEST_DIR` or CWD), looking for
   `.vcpkg-root` marker file — checks both `<dir>/vcpkg/.vcpkg-root` and
   `<dir>/.vcpkg-root`

### Re-exports

`lib.rs` re-exports all public types from `cmake_package`:

- `find_cmake`, `CMakePackage`, `CMakeTarget`, `CMAKE_MIN_VERSION`
- `Error`, `Version`, `VersionError`

This allows downstream users to work with cmake-package types without adding
it as a direct dependency.

## Development

### Prerequisites

- Rust toolchain (stable)
- [vcpkg](https://vcpkg.io/) ([install guide](https://learn.microsoft.com/vcpkg/get_started/get-started), for running integration tests)
- [reuse-tool](https://github.com/fsfe/reuse-tool) (for license compliance)

### Building

```bash
git clone https://github.com/black-desk/vcpkg-manifest-rs.git
cargo build
```

### Running Tests

Integration tests require vcpkg to be installed and `VCPKG_ROOT` to be set:

```bash
export VCPKG_ROOT=/path/to/vcpkg
cargo test --test integration_test
```

Integration tests will be skipped automatically if vcpkg is not available.

### Test Fixtures

The `tests/fixtures/hello-fmt/` directory contains a minimal project that:

- Uses `vcpkg_manifest::find_package("fmt")` in its `build.rs`
- Compiles a C++ wrapper (`src/wrapper.cpp`) via the `cc` crate, using
  include directories from the found package
- Links against the fmt library
- Calls `fmt::print()` from Rust via FFI

The fixture has its own `vcpkg.json` (`{"dependencies": ["fmt"]}`), a
pre-populated `vcpkg_installed/` directory, and a `target/` directory with
build artifacts committed (since the fixture is not in a gitignore).

### Adding a New Test Fixture

1. Create a new directory under `tests/fixtures/`
2. Add a `Cargo.toml` with `vcpkg-manifest` as a build-dependency (path
   relative to the fixture location)
3. Add a `vcpkg.json` declaring the C/C++ dependency
4. Write `build.rs`, `src/main.rs`, and any C/C++ wrapper files
5. Run `vcpkg install` in the fixture directory to populate `vcpkg_installed/`
6. Add test functions in `tests/integration_test.rs`
