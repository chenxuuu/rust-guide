use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    println!("  整数：   {}", rng.gen_range(0, 10)); // 半开放范围取随机值，即包括`低位`而不包括`高位`
    println!("  浮点数： {}", rng.gen_range(0.0, 10.0));
}
