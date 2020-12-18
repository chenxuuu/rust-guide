# Rust 实践指南

Rust 实践指南 - The Hitchhiker's Guide to Rust

如果欲了解本书，请阅读[本书前言](https://rust-guide.budshome.com/1-preface.html)。

## 在线阅读

《Rust 实践指南》，内容规划中。欢迎您提出宝贵意见：linshi@budshome.com，budshome（微信）。

在线试读地址：[http://rust-guide.budshome.com](http://rust-guide.budshome.com)。

## 离线阅读

如果你喜欢本地阅读方式，可以使用 mdBook（[中文文档](https://mdbook.budshome.com)） 进行书籍构建：

```bash
$ git clone https://github.com/zzy/rust-guide
$ cd rust-guide
$ cargo install mdbook # 请使用你感兴趣的版本参数，如：--vers "0.3.5"
$ mdbook serve --open # 或者 mdbook build
```

也可以直接用你喜欢的浏览器从 `book` 子目录打开 `index.html` 文件。

```bash
$ xdg-open ./book/index.html # linux
$ start .\book\index.html    # windows
$ open ./book/index.html     # mac
```

## 构建和测试

- 本书使用 mdBook（[中文文档](https://mdbook.budshome.com)） 进行构建。
- 实践实例放在 examples 目录中，请提交前进行测试。

## 贡献

《Rust 实践指南》的目的是让 Rust 程序员新手能够更容易地参与到 Rust 语言社区中，因此非常欢迎您的参与。

祝您学习愉快，欢迎提交问题，欢迎发送 PR。
