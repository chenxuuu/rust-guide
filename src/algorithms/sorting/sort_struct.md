### 结构体 Vector 排序

<!--
> [algorithms/sorting/sort_struct.md](https://github.com/zzy/rust-cookbook-zh-cn/blob/master/src/algorithms/sorting/sort_struct.md)
> <br />
> commit - eeb42dbc6f799a60df53fc1e7f214a5c01e9618d - 2020.09.07
-->

[![std-badge]][std] [![cat-science-badge]][cat-science]

依据自然顺序（按名称和年龄），对具有 `name` 和 `age` 属性的 Person 结构体 Vector 排序。为了使 Person 可排序，你需要四个 traits：[`Eq`]、[`PartialEq`]、[`Ord`]，以及 [`PartialOrd`]。这些 traits 可以被简单地派生。你也可以使用 [`vec:sort_by`] 方法自定义比较函数，仅按照年龄排序。

- `Eq`：对等式进行等价关系比较的 trait。这意味着，除了 `a == b` 和 `a != b` 严格可逆比较外，等式必须为（对于所有 `a`、`b` 和 `c` 而言）：
  - 自反：`a == a`；
  - 对称：`a == b` 意味着 `b == a`；以及
  - 等量代换：`a == b` 和 `b == c` 意味着 `a == c`。
- `PartialEq`：对等式部分进行等价关系比较的 trait。对于没有完全等价关系的类型，实现此 trait 允许部分相等。例如，在浮点数中，`NaN != NaN`，所以浮点类型实现 `PartialEq` 而不是 `Eq`。从形式上讲，等式必须为（对于所有 `a`、`b` 和 `c` 而言）：
  - 对称：`a == b` 意味着 `b == a`；以及
  - 等量代换：`a == b` 和 `b == c` 意味着 `a == c`。
- `Ord`：用于构成全序关系类型的 trait。如果是全序关系的排序（对于所有 `a`、`b` 和 `c` 而言）：
  - 完全非对称：`a < b`，`a == b` 或 `a > b` 中的一个为真；以及
  - 等量代换：`a < b` 和 `b < c` 意味着 `a < c`。对于 `==` 和 `>`，同样具有等量代换关系。
- `PartialOrd`：可比较排序规则的 trait。对于所有 `a`、`b` 和 `c` 而言，比较关系必须满足：
  - 非对称：如果 `a < b`，那么 `!(a > b)`，以及 `a > b` 意味着 `!(a < b)`；以及
  - 等量代换：`a < b` 和 `b < c` 意味着 `a < c`。对于 `==` 和 `>`，同样具有等量代换关系。
- `vec::sort`：对切片进行排序，这种排序是稳定的（即不重新排序相等的元素）。在合适的场景，不稳定排序是首选的，因为它通常比稳定排序快，并且不分配辅助内存。

```rust,edition2018
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person {
            name,
            age
        }
    }
}

fn main() {
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];

    // 根据获得的自然顺序（name 和 age）对 people 进行排序
    people.sort();

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("John".to_string(), 1),
            Person::new("Zoe".to_string(), 25),
        ]);

    // 根据 age 值对 people 进行排序
    people.sort_by(|a, b| b.age.cmp(&a.age));

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("Zoe".to_string(), 25),
            Person::new("John".to_string(), 1),
        ]);

}
```

[`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html 
[`PartialEq`]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
[`Ord`]: https://doc.rust-lang.org/std/cmp/trait.Ord.html
[`PartialOrd`]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
[`vec:sort_by`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_by
