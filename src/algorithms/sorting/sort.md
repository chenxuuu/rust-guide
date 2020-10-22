### 整数 Vector 排序

<!--
> [algorithms/sorting/sort.md](https://github.com/zzy/rust-cookbook-zh-cn/blob/master/src/algorithms/sorting/sort.md)
> <br />
> commit - eeb42dbc6f799a60df53fc1e7f214a5c01e9618d - 2020.09.07
-->

[![std-badge]][std] [![cat-science-badge]][cat-science]

这个实例通过 [`vec::sort`] 对一个整数 Vector 进行排序。另一种方法是使用 [`vec::sort_unstable`]，后者运行速度更快一些，但不保持相等元素的顺序。

- `vec::sort`：对切片进行排序，这种排序是稳定的（即不重新排序相等的元素）。在合适的场景，不稳定排序是首选的，因为它通常比稳定排序快，并且不分配辅助内存。
- `vec::sort_unstable`：对切片进行排序，但可能不会保留相等元素的顺序。这种排序类型不甚稳定（即，可能重新排序相等的元素）。

```rust,edition2018
fn main() {
    let mut vec = vec![1, 5, 10, 2, 15];
    
    vec.sort();

    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
}
```

[`vec::sort`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort
[`vec::sort_unstable`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_unstable
