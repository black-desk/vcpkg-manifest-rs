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

en | [zh_CN](README.zh_CN.md)

> [!WARNING]
>
> This English README is translated from the Chinese version using LLM and may
> contain errors.

A `build.rs` library for finding C/C++ dependencies in Rust projects via
[vcpkg](https://vcpkg.io/) ([install guide](https://learn.microsoft.com/vcpkg/get_started/get-started)).

Inspired by [vcpkg-rs](https://github.com/mcgoo/vcpkg-rs) and
[vcpkg-rs-mf](https://github.com/crosscode-nl/vcpkg-rs-mf). Both projects are
poorly maintained and have significant issues in practice, which motivated the
creation of this project.

When a `vcpkg_installed/` directory exists in the project root (created by
`vcpkg install`), it finds dependencies from vcpkg; otherwise, it falls back
to system-wide search. Both cases delegate to [cmake-package-rs] via CMake's
`find_package()`. See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed design
documentation.

[cmake-package-rs]: https://github.com/danvratil/cmake-package-rs

## Example

See [`tests/fixtures/hello-fmt/`](tests/fixtures/hello-fmt/) for a complete
example that uses `vcpkg_manifest::find_package("fmt")` to link the fmt
library provided by vcpkg.

## Known Issues

- **On Windows, dynamic linking is always used.** See [cmake-package-rs#123](https://codeberg.org/danvratil/cmake-package-rs/pulls/123) for details.

## Differences from Reference Projects

| Feature | [vcpkg-rs] | [vcpkg-rs-mf] | This project |
| --- | --- | --- | --- |
| Find libraries from vcpkg | ✓ | ✓ | ✓ |
| vcpkg manifest mode install paths | ✗ | ✓ | ✓ |

[vcpkg-rs]: https://github.com/mcgoo/vcpkg-rs
[vcpkg-rs-mf]: https://github.com/crosscode-nl/vcpkg-rs-mf

## License

Unless otherwise specified, the code of this project is open source under the
GNU General Public License version 3 or any later version, while documentation,
configuration files, and scripts used in the development and maintenance process
are open source under the MIT License.

This project complies with the [REUSE specification].

You can use [reuse-tool](https://github.com/fsfe/reuse-tool) to generate the
SPDX list for this project:

```bash
reuse spdx
```

[REUSE specification]: https://reuse.software/spec-3.3/
