// SPDX-FileCopyrightText: 2026 Chen Linxuan <me@black-desk.cn>
//
// SPDX-License-Identifier: GPL-3.0-or-later

extern "C" {
    fn hello_fmt() -> std::ffi::c_int;
}

fn main() {
    unsafe {
        hello_fmt();
    }
}
