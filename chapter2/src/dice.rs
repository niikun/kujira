// Rust でサイコロを振る

use rand::Rng;

pub fn run() {
    let mut rng = rand::thread_rng();
    for _ in 0..5 {
        let dice = rng.gen_range(1..=6);
        println!("サイコロの目: {}", dice);
    }
}
