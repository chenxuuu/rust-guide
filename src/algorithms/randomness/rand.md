### 生成随机数

<!--
> [algorithms/randomness/rand.md](https://github.com/zzy/rust-cookbook-zh-cn/blob/master/src/algorithms/randomness/rand.md)
> <br />
> commit - 34d313413dfea8bf2c50bf3c536a493cf28dea1b - 2020.09.11
-->

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

在随机数生成器 [`rand::Rng`] 的帮助下，通过 [`rand::thread_rng`] 生成随机数。可以开启多个线程，每个线程都有一个初始化的生成器。整数在其类型范围内均匀分布，浮点数是从 0 均匀分布到 1，但不包括 1。

- `rand::Rng`：`RngCore` 上自动实现的扩展 trait，为抽样取值和其它便捷方法实现了高层次的泛型方法。
- `rand::thread_rng`：创建随机数生成器线程的函数。调用后，由系统创建延迟初始化的本地线程。随机生成器线程将用于方法链（method chaining）样式，如 `thread_rng().gen::<i32>()`。或本地缓存，如 `let mut rng = thread_rng();`。由 `Default` trait 调用，等效于 `ThreadRng::default()`。

```rust,edition2018
use rand::Rng; // `RngCore` 上自动实现的扩展 trait，实现了高层次的泛型方法

fn main() {
    let mut rng = rand::thread_rng(); // 由系统创建延迟初始化的随机数生成器本地线程

    let n1: u8 = rng.gen(); // 返回一个支持标准分布的随机值
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}
```

[`rand::Rng`]: https://docs.rs/rand/*/rand/trait.Rng.html
[`rand::thread_rng`]: https://docs.rs/rand/*/rand/fn.thread_rng.html
