<!--
SPDX-FileCopyrightText: 2026 Chen Linxuan <me@black-desk.cn>

SPDX-License-Identifier: MIT
-->

# 贡献指南

[en](CONTRIBUTING.md) | zh_CN

感谢你对本项目的关注！本文档涵盖项目架构、设计决策和开发流程，帮助你快速上手。

## 项目概览

vcpkg-manifest-rs 提供一个用于在 `build.rs` 中查找 C/C++ 依赖的库（crate
`vcpkg-manifest`），支持 vcpkg 和系统范围搜索。实际的 CMake 包发现委托给
[cmake-package-rs](https://github.com/danvratil/cmake-package-rs)。

## 项目结构

```
vcpkg-manifest-rs
├── src/
│   └── lib.rs            # vcpkg_manifest 库 + FindPackageBuilder
├── tests/
│   ├── integration_test.rs  # 集成测试
│   └── fixtures/
│       └── hello-fmt/    # 测试夹具（Rust + C++，通过 vcpkg 使用 fmt 库）
└── .github/workflows/    # CI 和 CD 流水线
```

### 依赖关系

```
下游 crate 的 build.rs
    └── vcpkg_manifest (lib.rs)
            └── cmake-package
```

## 设计细节

### vcpkg 检测策略

库通过检查项目根目录（`CARGO_MANIFEST_DIR`）下是否存在
`vcpkg_installed/` 目录来判断是否使用 vcpkg。该目录由 vcpkg manifest
模式下的 `vcpkg install` 创建。

检测到时，`find_package()` 会配置 CMake 使用 vcpkg 的工具链文件；否则
回退到系统范围的 CMake 搜索。这意味着库在有或没有 vcpkg 的环境下都能
透明工作。

### VCPKG_ROOT 发现

`find_vcpkg_root()` 按以下顺序查找 vcpkg 安装：

1. `VCPKG_ROOT` 环境变量
2. 从 `CARGO_MANIFEST_DIR` 或当前工作目录开始，向上搜索父目录，查找
   `.vcpkg-root` 标记文件——同时检查 `<dir>/vcpkg/.vcpkg-root` 和
   `<dir>/.vcpkg-root`

### Re-exports

`lib.rs` re-exports 了 `cmake_package` 的所有公共类型：

- `find_cmake`、`CMakePackage`、`CMakeTarget`、`CMAKE_MIN_VERSION`
- `Error`、`Version`、`VersionError`

这使得下游用户可以直接使用 cmake-package 的类型，而无需将其添加为
直接依赖。

## 开发

### 前置条件

- Rust 工具链（stable）
- [vcpkg](https://vcpkg.io/)（[安装教程](https://learn.microsoft.com/vcpkg/get_started/get-started)，用于运行集成测试）
- [reuse-tool](https://github.com/fsfe/reuse-tool)（用于许可证合规检查）

### 构建

```bash
git clone https://github.com/black-desk/vcpkg-manifest-rs.git
cargo build
```

### 运行测试

集成测试需要安装 vcpkg 并设置 `VCPKG_ROOT`：

```bash
export VCPKG_ROOT=/path/to/vcpkg
cargo test --test integration_test
```

如果 vcpkg 不可用，集成测试会自动跳过。

### 测试夹具

`tests/fixtures/hello-fmt/` 目录包含一个最小项目，它：

- 在 `build.rs` 中使用 `vcpkg_manifest::find_package("fmt")`
- 通过 `cc` crate 编译 C++ 包装文件（`src/wrapper.cpp`），使用找到的
  包的 include 目录
- 链接 fmt 库
- 通过 FFI 从 Rust 调用 `fmt::print()`

夹具有自己的 `vcpkg.json`（`{"dependencies": ["fmt"]}`）、预填充的
`vcpkg_installed/` 目录，以及已提交的 `target/` 构建产物（因为夹具
不在 .gitignore 中）。

### 添加新的测试夹具

1. 在 `tests/fixtures/` 下创建新目录
2. 添加 `Cargo.toml`，将 `vcpkg-manifest` 作为 build-dependency（路径
   相对于夹具位置）
3. 添加声明 C/C++ 依赖的 `vcpkg.json`
4. 编写 `build.rs`、`src/main.rs` 和任何 C/C++ 包装文件
5. 在夹具目录中运行 `vcpkg install` 以填充 `vcpkg_installed/`
6. 在 `tests/integration_test.rs` 中添加测试函数
