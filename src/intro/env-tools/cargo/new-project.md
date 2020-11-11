# 使用 Cargo 创建项目

## cargo new，创建项目

使用 Cargo 创建项目，执行 `cargo new` 命令:

```shell
$ cargo new hello_world --bin
```

我们传递参数 `--bin`，是因为我们将创建一个二进制程序（默认）。如果我们创建一个库（lib），就需要传递参数 `--lib`。

默认情况下，新创建项目目录会初始化为一个 `git` 仓库，如果你不希望初始化为 `git` 仓库，需要传递参数 `--vcs none`。

上述命令执行后，Cargo 会创建以下文件：

```shell
$ cd hello_world
$ tree .
.
├── Cargo.toml
└── src
    └── main.rs

1 directory, 2 files
```

然后，让我们看看 `Cargo.toml` 文件：

```toml
[package]
name = "hello_world"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]
```

此文件被称作为 **manifest** 元清单，它包含了 Cargo 编译项目所需的所有元数据。

接下来，我们阅读 `src/main.rs` 源文件：

```rust
fn main() {
    println!("Hello, world!");
}
```

## cargo build，编译项目

Cargo 为我们创造了一个 "hello_world"，我们通过 `cargo build` 命令来编译它：

```shell
$ cargo build
   Compiling hello_world v0.1.0 (file:///path/to/project/hello_world)
```

Cargo 将我们的代码编译为可执行文件。目前，我们使用的是默认的调试模式进行编译，因此要运行此示例代码，执行 `./target/debug` 目录下的二进制文件：

```shell
$ ./target/debug/hello_world
Hello, world!
```

## cargo run，编译并运行项目

我们也可以直接使用 `cargo run` 命令来运行源代码。`cargo run` 命令会自行编译，然后运行它：

```shell
$ cargo run
     Fresh hello_world v0.1.0 (file:///path/to/project/hello_world)
   Running `target/hello_world`
Hello, world!
```

执行 `cargo build` 命令后，你会注意到，项目目录中创建了几个新文件和目录：

```shell
$ tree .
.
|-- Cargo.lock
|-- Cargo.toml
|-- src
|   `-- main.rs
`-- target
    `-- debug
        |-- build
        |-- deps
        |   |-- hello_world-6ad0b2df81336e7f
        |   |-- hello_world-6ad0b2df81336e7f.d
        |   `-- hello_world-6ad0b2df81336e7f.dSYM
        |       `-- Contents
        |           |-- Info.plist
        |           `-- Resources
        |               `-- DWARF
        |                   `-- hello_world-6ad0b2df81336e7f
        |-- examples
        |-- hello_world
        |-- hello_world.d
        |-- hello_world.dSYM -> deps/hello_world-6ad0b2df81336e7f.dSYM
        |-- incremental
        |   // ...
        `-- native

15 directories, 19 files
```

其中的 `Cargo.lock` 文件，包含项目依赖项的有关信息（即使还未有依赖，此文件也会在编译后产生），其内容可读性较差。另外，`target` 目录包含所有构建产品（二进制文件、依赖项编译文件等)。并且，如上文 `cargo build` 命令执行时所提及：Cargo 默认生成调试（debug）版本。

## cargo build --release，发布项目

你可以使用 `cargo build --release`，这会在开启优化的情况下，编译文件：

```shell
$ cargo build --release
   Compiling hello_world v0.1.0 (file:///path/to/project/hello_world)
```

`cargo build --release` 执行后，产生的二进制文件将放入目录 `target/release`，而不再是目录 `target/debug`。

使用调试模式（debug）进行编译，是 Rust 开发的默认设置。因为调试模式下的编译过程中，编译器不进行优化，因此其编译时间较短。但代码编译后产生的二进制可执行文件，其运行速度会较慢。

使用发布模式（release）进行编译，会需要更多的时间，但代码编译后产生的二进制可执行文件，其运行速度会更快。

> 关于 Cargo 的使用，我们会在`附录二：Cargo 进阶`再做介绍。
