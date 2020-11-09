# 环境和工具

## 运行环境

Rust 可以达到快速、跨平台、低资源占用的目的，目前全世界在生产环境使用 Rust 的公司已经很多，并且增长速度非常快。从初创公司到大型企业，从嵌入式设备到可扩展的 Web 服务，Rust 都完全合适。虽然并无直接的因果关系，但我们也可大概推导出，Rust 对于环境的适配能力是非常强的，可以适配于各种操作系统和平台。

包括 Rust 官方组织、开源贡献者，以及业界公司，目前的 Rust 开发、测试，以及生产环境，多以 Linux 环境为主。因此本书中开发环境也以 Linux 环境为主，具体为 `Ubuntu Server 18.04.5 LTS`。对于其它 Linux 发行版，Rust 安装成功后，具体开发、调试，以及发布的命令，均没有差别。

书中对于 macOS、Windows 环境下的 Rust 开发、调试，以及运行，也会有涉及，并且有详细的讲解。

总体来说，Rust 是`跨平台`的系统级编程语言，对于应用环境的适配能力非常强，不用开发者、维护者花费过多精力。

## 构建工具

本书中，我们使用 Rust 官方提供的非常强大的构建系统和包管理器 `Cargo`。它可以为你处理很多任务，比如构建代码、下载依赖库并编译这些库。

## 开发工具

对于开发者来说，趁手的代码编辑器、IDE，对于开发、调试过程有极大的帮助，可以极大程度提高开发效率。本书中，对于 Rust 的开发工具介绍主要是在代码编辑器、IDE 方面给予简洁的说明，辅助读者比较选择。

具体来讲，本章节内容划分为如下：

- [安装、更新，和卸载](env-tools/installation.md)
  - [Linux/WSL/macOS 配置 Rust 环境](env-tools/installation/linux-wsl-macos.md)
  - [Windows 配置 Rust 环境](env-tools/installation/windows.md)
- [构建工具 Cargo](env-tools/cargo.md)
  - [使用 Cargo 创建项目](env-tools/cargo/new-project.md)
  - [配置 Cargo 国内镜像源](env-tools/cargo/source-replacement.md)
- [编辑器及 IDE](env-tools/editor-ide.md)
