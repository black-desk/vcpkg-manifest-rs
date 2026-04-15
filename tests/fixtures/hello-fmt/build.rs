// SPDX-FileCopyrightText: 2026 Chen Linxuan <me@black-desk.cn>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::env;
use std::path::PathBuf;

fn main() {
    let pkg = vcpkg_manifest::find_package("fmt")
        .find()
        .expect("Failed to find fmt package");

    let target = pkg
        .target("fmt::fmt")
        .expect("fmt::fmt target not found");

    // Compile the C++ wrapper, passing include dirs from the found package
    let mut build = cc::Build::new();
    build.cpp(true);
    build.file("src/wrapper.cpp");

    for dir in &target.include_directories {
        build.include(dir);
    }

    build.compile("hello_fmt_wrapper");

    // Link the fmt library (emits cargo:rustc-link-* directives)
    target.link();

    // Tell cargo to rerun if vcpkg_installed changes
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    if manifest_dir.join("vcpkg_installed").is_dir() {
        println!("cargo:rerun-if-changed=vcpkg_installed");
    }
}
