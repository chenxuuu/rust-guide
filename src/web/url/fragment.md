## 从 URL 移除片段标识符和查询对

[![url-badge]][url] [![cat-net-badge]][cat-net]

解析 [`Url`] 结构体，并使用 [`url::Position`] 枚举对其进行切片，以去除不需要的 URL 片段。

```rust,edition2018


use url::{Url, Position, ParseError};

fn main() -> Result<(), ParseError> {
    let parsed = Url::parse("https://github.com/rust-lang/rust/issues?labels=E-easy&state=open")?;
    let cleaned: &str = &parsed[..Position::AfterPath];
    println!("cleaned: {}", cleaned);
    Ok(())
}
```

[`url::Position`]: https://docs.rs/url/*/url/enum.Position.html
[`Url`]: https://docs.rs/url/*/url/struct.Url.html
