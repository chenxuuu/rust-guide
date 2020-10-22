### 生成自定义类型随机值

<!--
> [algorithms/randomness/rand-custom.md](https://github.com/zzy/rust-cookbook-zh-cn/blob/master/src/algorithms/randomness/rand-custom.md)
> <br />
> commit - 1758f63077836b734be0d62c550403c220056aa2 - 2020.09.06
-->

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

随机生成一个元组 `(i32, bool, f64)` 和用户定义类型为 `Point` 的变量。为  [`Standard`]  实现 [`Distribution`] trait，以允许随机生成。

- `Standard`：通用的随机值分布结构体，为 rand crate 中的众多基本类型实现。通常生成均匀分布的数值，并且具有与类型相适应的范围。
- `Distribution`：创建概率分布类型的 trait，可以用来创建泛型 T 的随机实例的类型。提供 `sample_iter` 方法，该方法生成一个迭代器，从分布中采样。

```rust,edition2018
use rand::Rng;
use rand::distributions::{Distribution, Standard};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();
    println!("Random tuple: {:?}", rand_tuple);
    println!("Random Point: {:?}", rand_point);
}
```

[`Distribution`]: https://docs.rs/rand/*/rand/distributions/trait.Distribution.html
[`Standard`]: https://docs.rs/rand/*/rand/distributions/struct.Standard.html
