use rand_distr::{Distribution, Normal, NormalError};
use rand::thread_rng;

fn main() -> Result<(), NormalError> {
    let mut rng = thread_rng();
    let normal = Normal::new(2.0, 9.0)?; // 正态分布
    let v = normal.sample(&mut rng);

    println!("  正态分布： {}", v);

    Ok(())
}
