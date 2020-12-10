# 4. Cargo 简介

Rust 官方提供了非常强大的构建系统和包管理器 `Cargo`。Cargo 可以为您处理很多任务，比如下载 crate 依赖项、编译 crate、构建代码、生成可分发的 crate，并将它们上传到 crates.io —— Rust 社区的 crate 注册表。

> Rust 中的 crate，类似于其它编程语言中的`“包”或者“库”`。目前，Rust 中约定不做翻译。

Rust 和 Cargo 捆绑，或者说 Rust 安装器中内置了 Cargo。因此，成功安装 Rust 之后，Cargo 也会随之安装，并主动配置环境变量。

> 注：本章中对 Cargo 工具的介绍比较简略，仅限于创建、编译、调试，以及运行本书中的实例。在附录章节 [24.2. 附录二：Cargo 进阶](./24-appendix/24.2-cargo.md)中对 Cargo 做了进一步介绍，您也可以查阅 [Cargo 中文文档](https://cargo.budshome.com)以对 Cargo 进行全面了解。
