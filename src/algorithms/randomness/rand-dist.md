### 生成给定分布随机数

<!--
> [algorithms/randomness/rand-dist.md](https://github.com/zzy/rust-cookbook-zh-cn/blob/master/src/algorithms/randomness/rand-dist.md)
> <br />
> commit - e7c4a93116ef1a0dc50a526b81e9633770cc2afa - 2020.09.07
-->

[![rand_distr-badge]][rand_distr] [![cat-science-badge]][cat-science]

默认情况下，随机数在 `rand` crate 中是[均匀分布][uniform distribution]。[`rand_distr`] crate 提供其它的分布类型。如要使用，首先实例化一个分布，然后在随机数生成器 [`rand::Rng`] 的帮助下，使用 [`Distribution::sample`] 从该分布中进行采样。

- 均匀分布：在概率论和统计学中，均匀分布也叫矩形分布，它是对称概率分布，在相同长度间隔的分布概率是可能相等的。
- `rand_distr`：rand_distr crate 是 `rand::distributions` 模块的一个超级集合，实现了诸多概率分布类型，如均匀分布、正态分布（Normal distribution）、柯西分布（Cauchy distribution）等。请参考本章 “crate 介绍”一节。
- `Distribution::sample`：此函数创建一个迭代器，用来生成泛型 T 的随机值，其使用 `rng` 作为随机来源。

关于更多信息，阅读[可用分布文档][rand-distributions]。如下是一个使用[`正态（Normal）`][`Normal`]分布的实例。

```rust,edition2018,ignore
use rand_distr::{Distribution, Normal, NormalError};
use rand::thread_rng;

fn main() -> Result<(), NormalError> {
    let mut rng = thread_rng();
    let normal = Normal::new(2.0, 3.0)?; // 正态分布
    let v = normal.sample(&mut rng);
    println!("{} is from a N(2, 9) distribution", v);
    Ok(())
}
```

[`Distribution::sample`]: https://docs.rs/rand/*/rand/distributions/trait.Distribution.html#tymethod.sample
[`Normal`]: https://docs.rs/rand_distr/*/rand_distr/struct.Normal.html
[`rand::Rng`]: https://docs.rs/rand/*/rand/trait.Rng.html
[`rand_distr`]: https://docs.rs/rand_distr/*/rand_distr/index.html
[rand-distributions]: https://docs.rs/rand_distr/*/rand_distr/index.html
[uniform distribution]: https://en.wikipedia.org/wiki/Uniform_distribution_(continuous)
