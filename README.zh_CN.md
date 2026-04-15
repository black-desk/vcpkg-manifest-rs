<!--
SPDX-FileCopyrightText: 2026 Chen Linxuan <me@black-desk.cn>

SPDX-License-Identifier: MIT
-->

# vcpkg-manifest-rs

[![checks][badge-shields-io-checks]][actions]
[![commit activity][badge-shields-io-commit-activity]][commits]
[![contributors][badge-shields-io-contributors]][contributors]
[![release date][badge-shields-io-release-date]][releases]
![commits since release][badge-shields-io-commits-since-release]
[![codecov][badge-shields-io-codecov]][codecov]

[badge-shields-io-checks]:
  https://img.shields.io/github/check-runs/black-desk/vcpkg-manifest-rs/master

[actions]: https://github.com/black-desk/vcpkg-manifest-rs/actions

[badge-shields-io-commit-activity]:
  https://img.shields.io/github/commit-activity/w/black-desk/vcpkg-manifest-rs/master

[commits]: https://github.com/black-desk/vcpkg-manifest-rs/commits/master

[badge-shields-io-contributors]:
  https://img.shields.io/github/contributors/black-desk/vcpkg-manifest-rs

[contributors]: https://github.com/black-desk/vcpkg-manifest-rs/graphs/contributors

[badge-shields-io-release-date]:
  https://img.shields.io/github/release-date/black-desk/vcpkg-manifest-rs

[releases]: https://github.com/black-desk/vcpkg-manifest-rs/releases

[badge-shields-io-commits-since-release]:
  https://img.shields.io/github/commits-since/black-desk/vcpkg-manifest-rs/latest

[badge-shields-io-codecov]:
  https://codecov.io/github/black-desk/vcpkg-manifest-rs/graph/badge.svg?token=6TSVGQ4L9X
[codecov]: https://codecov.io/github/black-desk/vcpkg-manifest-rs

[en](README.md) | zh_CN

一个用于在 Rust 项目中通过 [vcpkg](https://vcpkg.io/)（[安装教程](https://learn.microsoft.com/vcpkg/get_started/get-started)） 查找 C/C++ 依赖的
`build.rs` 库。

参考了 [vcpkg-rs](https://github.com/mcgoo/vcpkg-rs) 和
[vcpkg-rs-mf](https://github.com/crosscode-nl/vcpkg-rs-mf) 的实现。这两个项目
长期缺乏维护，使用中存在较多问题，因此发起了本项目。

当项目根目录存在 `vcpkg_installed/` 目录时（由 `vcpkg install` 创建），
从 vcpkg 安装中查找依赖；否则 fallback 到系统范围内查找。底层均委托
[cmake-package-rs] 通过 CMake `find_package()` 完成。详细设计文档见
[CONTRIBUTING.zh_CN.md](CONTRIBUTING.zh_CN.md)。

[cmake-package-rs]: https://github.com/danvratil/cmake-package-rs

## 示例

参见 [`tests/fixtures/hello-fmt/`](tests/fixtures/hello-fmt/)，其中包含一个完整示例，
演示了如何通过 `vcpkg_manifest::find_package("fmt")` 链接 vcpkg 提供的 libfmt。

## 已知问题

- **在 Windows 上总会使用动态链接。** 详情参见 [cmake-package-rs#123](https://codeberg.org/danvratil/cmake-package-rs/pulls/123)。

## 与参考项目的区别

| 特性 | [vcpkg-rs] | [vcpkg-rs-mf] | 本项目 |
| --- | --- | --- | --- |
| 从 vcpkg 查找库 | ✓ | ✓ | ✓ |
| vcpkg manifest mode 安装路径 | ✗ | ✓ | ✓ |

[vcpkg-rs]: https://github.com/mcgoo/vcpkg-rs
[vcpkg-rs-mf]: https://github.com/crosscode-nl/vcpkg-rs-mf

## 许可证

如无特殊说明，该项目的代码以GNU通用公共许可协议第三版或任何更新的版本开源，文档、配置文件以及开发维护过程中使用的脚本等以MIT许可证开源。

该项目遵守[REUSE规范]。

你可以使用[reuse-tool](https://github.com/fsfe/reuse-tool)生成这个项目的SPDX列表：

```bash
reuse spdx
```

[REUSE规范]: https://reuse.software/spec-3.3/
