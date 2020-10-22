### 浮点数 Vector 排序

<!--
> [algorithms/sorting/sort_float.md](https://github.com/zzy/rust-cookbook-zh-cn/blob/master/src/algorithms/sorting/sort_float.md)
> <br />
> commit - 1758f63077836b734be0d62c550403c220056aa2 - 2020.09.06
-->

[![std-badge]][std] [![cat-science-badge]][cat-science]

f32 或 f64 的 vector，可以使用 [`vec::sort_by`] 和 [`PartialOrd::partial_cmp`] 对其进行排序。

- `vec::sort_by`：使用比较器（comparator ）函数对切片进行排序，这种排序是稳定的（即不重新排序相等的元素）。比较器（comparator ）函数必须为切片中的元素定义一个总顺序。如果不是全序关系排序，则不指定元素的顺序。如果排序是全序关系（对于所有 `a`、`b` 和 `c`），则：
  - 完全反对称：`a < b`，`a == b` 或 `a > b` 中的一个为真，并且
  - 等量代换：`a < b` 和 `b < c` 意味着 `a < c`。对于 `==` 和 `>`，同样具有等量代换关系。
- `PartialOrd::partial_cmp`：如果存在`其它值（other ）`，此方法将返回`自己（self）`和`其它值（other ）`之间的排序。

```rust,edition2018
fn main() {
    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];

    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]);
}
```

[`vec::sort_by`]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by
[`PartialOrd::partial_cmp`]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html#tymethod.partial_cmp
