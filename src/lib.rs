// SPDX-FileCopyrightText: 2026 Chen Linxuan <me@black-desk.cn>
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! A build.rs library for finding C/C++ dependencies via vcpkg or system-wide search.
//!
//! Built on top of [`cmake_package`]. When a `vcpkg_installed/` directory is detected
//! in the project root (created by `vcpkg install` or `cargo vcpkg-build`), it sets
//! the `CMAKE_TOOLCHAIN_FILE` environment variable so that CMake finds packages from
//! the vcpkg installation. Otherwise, it falls back to system-wide search via
//! [`cmake_package`].
//!
//! # Example
//!
//! ```ignore
//! fn main() {
//!     let target = vcpkg_manifest::find_package("OpenSSL")
//!         .find()
//!         .expect("OpenSSL not found")
//!         .target("OpenSSL::SSL")
//!         .expect("OpenSSL::SSL target not found");
//!     target.link();
//! }
//! ```

use std::env;
use std::path::{Path, PathBuf};

// Re-export cmake_package types for convenience
pub use cmake_package::{find_cmake, CMakePackage, CMakeTarget, CMAKE_MIN_VERSION};
pub use cmake_package::{Error, Version, VersionError};

/// The directory name for vcpkg installed packages in manifest mode.
const VCPKG_INSTALLED_DIR: &str = "vcpkg_installed";

/// The vcpkg CMake toolchain file path relative to VCPKG_ROOT.
const VCPKG_CMAKE_TOOLCHAIN: &str = "scripts/buildsystems/vcpkg.cmake";

/// The marker file that identifies a vcpkg root directory.
const VCPKG_ROOT_MARKER: &str = ".vcpkg-root";

/// Find a CMake package using vcpkg (if available) or system-wide search.
///
/// Returns a [`FindPackageBuilder`] that can be used to configure the search
/// and then call [`FindPackageBuilder::find()`] to locate the package.
pub fn find_package(name: impl Into<String>) -> FindPackageBuilder {
    FindPackageBuilder {
        inner: cmake_package::find_package(name),
    }
}

/// A builder for finding a CMake package.
///
/// Wraps [`cmake_package::FindPackageBuilder`] with automatic vcpkg support.
/// When `find()` is called, it detects whether vcpkg is available and configures
/// the CMake toolchain accordingly.
pub struct FindPackageBuilder {
    inner: cmake_package::FindPackageBuilder,
}

impl FindPackageBuilder {
    /// Specifies the minimum required version for the package.
    pub fn version(self, version: impl TryInto<cmake_package::Version>) -> Self {
        Self {
            inner: self.inner.version(version),
        }
    }

    /// Specifies the required components to locate in the package.
    pub fn components(self, components: impl Into<Vec<String>>) -> Self {
        Self {
            inner: self.inner.components(components),
        }
    }

    /// Specifies alternative package names to search for.
    pub fn names<S, I>(self, names: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        Self {
            inner: self.inner.names(names),
        }
    }

    /// Enables verbose output from CMake.
    pub fn verbose(self) -> Self {
        Self {
            inner: self.inner.verbose(),
        }
    }

    /// Tries to find the CMake package.
    ///
    /// If `vcpkg_installed/` is detected in the project root, it configures
    /// the vcpkg CMake toolchain before delegating to [`cmake_package`].
    /// Otherwise, it performs a system-wide search.
    pub fn find(self) -> Result<CMakePackage, Error> {
        let Some(vcpkg_root) = detect_vcpkg_root() else {
            return self.inner.find();
        };
        return self
            .inner
            .define(
                "CMAKE_TOOLCHAIN_FILE",
                vcpkg_root.join(VCPKG_CMAKE_TOOLCHAIN).to_string_lossy(),
            )
            .define(
                "VCPKG_MANIFEST_DIR",
                std::env::var("CARGO_MANIFEST_DIR").unwrap(),
            )
            .define(
                "VCPKG_INSTALLED_DIR",
                std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/vcpkg_installed",
            )
            .find();
    }
}

/// Detect whether vcpkg is in use by checking for `vcpkg_installed/` directory.
///
/// Looks for `vcpkg_installed/` starting from the directory containing
/// `CARGO_MANIFEST_DIR`.
fn has_vcpkg_installed() -> bool {
    let manifest_dir = match env::var("CARGO_MANIFEST_DIR") {
        Ok(dir) => PathBuf::from(dir),
        Err(_) => return false,
    };
    manifest_dir.join(VCPKG_INSTALLED_DIR).is_dir()
}

/// Find the VCPKG_ROOT directory.
///
/// Search order:
/// 1. `VCPKG_ROOT` environment variable
/// 2. Search parent directories for `vcpkg/.vcpkg-root` (from `CARGO_MANIFEST_DIR`
///    or current working directory)
fn find_vcpkg_root() -> Option<PathBuf> {
    // 1. Check VCPKG_ROOT environment variable
    if let Ok(root) = env::var("VCPKG_ROOT") {
        let path = PathBuf::from(&root);
        if path.is_dir() {
            return Some(path);
        }
    }

    // 2. Search parent directories for vcpkg installation
    let start_dir = env::var("CARGO_MANIFEST_DIR")
        .ok()
        .map(PathBuf::from)
        .or_else(|| env::current_dir().ok())?;
    search_ancestors_for_vcpkg(&start_dir)
}

/// Search ancestor directories for a vcpkg installation.
///
/// Looks for `vcpkg/.vcpkg-root` or `.vcpkg-root` in each ancestor directory.
fn search_ancestors_for_vcpkg(start: &Path) -> Option<PathBuf> {
    let mut current = start;
    loop {
        // Check for vcpkg/.vcpkg-root (common layout)
        if current.join("vcpkg").join(VCPKG_ROOT_MARKER).is_file() {
            return Some(current.join("vcpkg"));
        }
        // Check for .vcpkg-root directly (vcpkg is the current dir)
        if current.join(VCPKG_ROOT_MARKER).is_file() {
            return Some(current.to_path_buf());
        }
        current = current.parent()?;
    }
}

/// Detect vcpkg root only if vcpkg_installed/ exists.
fn detect_vcpkg_root() -> Option<PathBuf> {
    if !has_vcpkg_installed() {
        return None;
    }
    find_vcpkg_root()
}

