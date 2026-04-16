// SPDX-FileCopyrightText: 2026 Chen Linxuan <me@black-desk.cn>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::path::PathBuf;
use std::process::Command;

use assert_cmd::assert::OutputAssertExt;
use predicates::prelude::*;

/// Path to the hello-fmt fixture project.
fn fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("hello-fmt")
}

/// Check whether vcpkg is available (VCPKG_ROOT set or vcpkg on PATH).
fn vcpkg_available() -> bool {
    if ! std::env::var("VCPKG_ROOT").is_ok() {
        return false;
    }
    Command::new("vcpkg")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Run the fixture binary after it has been built.
fn run_fixture_binary() -> assert_cmd::assert::Assert {
    let bin_path = fixture_dir()
        .join("target")
        .join("debug")
        .join("hello-fmt");
    Command::new(bin_path).assert()
}

#[test]
fn test_find_package_with_vcpkg_installed() {
    if !vcpkg_available() {
        eprintln!("Skipping: vcpkg not available");
        return;
    }

    let fixture = fixture_dir();

    // Ensure vcpkg dependencies are installed first
    let vcpkg_root = std::env::var("VCPKG_ROOT").expect("VCPKG_ROOT not set");
    let vcpkg_status = Command::new("vcpkg")
        .arg("install")
        .current_dir(&fixture)
        .env("VCPKG_ROOT", &vcpkg_root)
        .status()
        .expect("Failed to run vcpkg install");

    assert!(vcpkg_status.success(), "vcpkg install failed");

    // Now just cargo build (find_package path, not cargo vcpkg-build)
    let build_status = Command::new("cargo")
        .arg("build")
        .current_dir(&fixture)
        .status()
        .expect("Failed to run cargo build");

    assert!(build_status.success(), "cargo build failed");

    // Run the built binary and verify output
    run_fixture_binary()
        .success()
        .stdout(predicate::str::contains("hello from fmt: 42"));
}
