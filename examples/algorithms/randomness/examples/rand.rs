use rand::Rng; // `RngCore` 上自动实现的扩展 trait，实现了高层次的泛型方法

fn main() {
    let mut rng = rand::thread_rng(); // 由系统创建的本地线程，是延迟初始化的随机数生成器

    let n1: u8 = rng.gen(); // 返回一个支持标准分布的随机值
    let n2: u16 = rng.gen();
    
    println!("  u8    随机数： {}", n1);
    println!("  u16   随机数： {}", n2);
    println!("  u32   随机数： {}", rng.gen::<u32>());
    println!("  i32   随机数： {}", rng.gen::<i32>());
    println!("  float 随机数： {}", rng.gen::<f64>());
}
