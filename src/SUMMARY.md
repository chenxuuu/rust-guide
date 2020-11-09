# Rust 生态实践指南

[Rust 生态实践指南](index.md)

------

<!-- - [前言](foreword.md) -->
- [自序](preface.md)
- [简介](intro.md)

------

- [环境和工具](env-tools.md)
  - [环境配置](env-tools/installation.md)
    - [Linux/WSL/macOS 配置 Rust 环境](env-tools/installation/linux-wsl-macos.md)
    - [Windows 配置 Rust 环境](env-tools/installation/windows.md)
  - [构建工具 Cargo](env-tools/cargo.md)
    - [使用 Cargo 创建项目](env-tools/cargo/new-project.md)
    - [配置 Cargo 国内镜像源](env-tools/cargo/source-replacement.md)
  - [编辑器及 IDE](env-tools/editor-ide.md)
- [算法](algorithms.md)
  - [生成随机值](algorithms/randomness.md)
  - [Vector 排序](algorithms/sorting.md)

------

- [附录](appendix.md)
  - [附录一：源码编译安装 Rust](appendix/installing-from-source.md)
  - [附录二：Cargo 进阶](appendix/cargo.md)
    - [Cargo 清单格式详解](appendix/cargo/manifest.md)
    - [Cargo.toml、依赖项管理，以及 Cargo.lock](appendix/cargo/cargo-toml-dependencies-cargo-lock.md)
    - [项目的构建、运行、调试，以及发布](appendix/cargo/build-run-debug-release.md)
    - [使用 Cargo 发布自己的 crate](appendix/cargo/publishing.md)
    - [Cargo 构建脚本](appendix/cargo/build-scripts.md)
    - [Cargo 外部工具](appendix/cargo/external-tools.md)
    - [Cargo 常见问题](appendix/cargo/faq.md)
  - [附录三：Rust 模糊测试](appendix/rust-fuzz.md)
    - [使用 cargo-fuzz 进行模糊测试](appendix/rust-fuzz/cargo-fuzz.md)
    - [使用 afl.rs 进行模糊测试](appendix/rust-fuzz/afl-rs.md)
  - [附录四：书籍构建工具 mdBook](appendix/mdbook.md)

------
