use rand::distributions::{Distribution, Uniform};

fn main() {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(&mut rng);

        println!("  丢一次骰子： {}", throw);
        
        if throw == 6 {
            break;
        }
    }
}
