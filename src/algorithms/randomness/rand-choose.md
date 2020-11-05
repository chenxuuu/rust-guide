### 从一组用户定义字符创建随机密码

[![rand-badge]][rand] [![cat-os-badge]][cat-os]

使用用户自定义的字节字符串，使用 [`gen_range`] 函数，随机生成一个给定长度的 ASCII 字符串。

- `gen_range`：此函数在 `[低位，高位]` 范围内生成一个随机值。此范围为半开放范围，即包括`低位`而不包括`高位`。此函数针对给定范围内仅生成一个样本值的情况进行了优化。另外，此函数为均匀分布类型，如果从相同的范围重复抽样取值，执行速度将会更快。

```rust,edition2018
fn main() {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 30;
    let mut rng = rand::thread_rng(); // 由系统创建延迟初始化的随机数生成器本地线程

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len()); // 半开放范围取随机值，即包括`低位`而不包括`高位`
            CHARSET[idx] as char
        })
        .collect();

    println!("{:?}", password);
}
```

[`gen_range`]: https://docs.rs/rand/*/rand/trait.Rng.html#method.gen_range
