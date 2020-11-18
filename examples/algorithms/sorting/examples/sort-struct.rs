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
        Person::new("Zhang".to_string(), 25),
        Person::new("Liu".to_string(), 60),
        Person::new("Wang".to_string(), 1),
    ];
    println!("  排序前： {:?}", people);

    // 根据获得的自然顺序（name 和 age）对 people 进行排序
    people.sort();
    println!("  排序后（name 和 age）： {:?}", people);

    assert_eq!(
        people,
        vec![
            Person::new("Liu".to_string(), 60),
            Person::new("Wang".to_string(), 1),
            Person::new("Zhang".to_string(), 25),
        ]);

    // 根据 age 值对 people 进行排序
    people.sort_by(|a, b| b.age.cmp(&a.age));
    println!("  排序后（age）： {:?}", people);

    assert_eq!(
        people,
        vec![
            Person::new("Liu".to_string(), 60),
            Person::new("Zhang".to_string(), 25),
            Person::new("Wang".to_string(), 1),
        ]);

}
