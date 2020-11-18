use rand::Rng;

fn main() {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 30;

    let mut rng = rand::thread_rng(); // 由系统创建延迟初始化的随机数生成器本地线程

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len()); // 半开放范围取随机值，即包括`低位`而不包括`高位`
            CHARSET[idx] as char
        })
        .collect();

    println!("  随机密码： {:?}", password);
}
