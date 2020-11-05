# Vector 排序

## crate 介绍

### std（The Rust Standard Library）

Rust 标准库是可移植的 Rust 软件的基础，它针对广泛的 Rust 生态系统，是其最核心的一组共享抽象，经过严格的测试和检验。

Rust 标准库提供了核心类型，如 `Vec<T>` 和 `Option<T>`、语言原语上的库定义操作、标准宏、I/O，以及多线程等。

`std` 默认适用于所有 Rust crate。因此，可以通过 `std` 路径，在 `use` 语句中访问标准库，就像 `use std::env` 一样。

## 实例实践

{{#include sorting/sort.md}}
{{#include sorting/sort_float.md}}
{{#include sorting/sort_struct.md}}

{{#include ../links.md}}
