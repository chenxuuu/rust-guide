## 解析 URL 字符串为 `Url` 类型

[![url-badge]][url] [![cat-net-badge]][cat-net]

`url` crate 中的 [`parse`] 方法验证并解析 `&str` 切片为 [`Url`] 结构体。如果输入字符串的格式不正确，解析方法 [`parse`] 会返回 `Result<Url, ParseError>`。

一旦 URL 被解析，它就可以使用 `Url` 结构体类型中的所有方法。

```rust,edition2018

use url::{Url, ParseError};

fn main() -> Result<(), ParseError> {
    let s = "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open";

    let parsed = Url::parse(s)?;
    println!("The path part of the URL is: {}", parsed.path());

    Ok(())
}
```

[`parse`]: https://docs.rs/url/*/url/struct.Url.html#method.parse
[`Url`]: https://docs.rs/url/*/url/struct.Url.html
