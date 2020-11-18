fn main() {
    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];
    println!("  排序前： {:?}", vec);

    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("  排序后： {:?}", vec);

    assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]);
}
