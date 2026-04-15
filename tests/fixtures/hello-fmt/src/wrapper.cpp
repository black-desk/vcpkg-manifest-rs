// SPDX-FileCopyrightText: 2026 Chen Linxuan <me@black-desk.cn>
//
// SPDX-License-Identifier: GPL-3.0-or-later

#include <fmt/core.h>

extern "C" int hello_fmt() {
    fmt::print("hello from fmt: {}\n", 42);
    return 0;
}
