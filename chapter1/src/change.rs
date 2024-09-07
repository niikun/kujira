fn main() {
    let price: i32 = 3950;
    let count500: i32 = 10;
    let count100: i32 = 3;
    let count50 = 10;
    let mut coins: Vec<(i32, i32, i32)> = Vec::new();

    for i in 0..=count500 {
        for j in 0..=count100 {
            for k in 0..=count50 {
                if price == (i * 500 + j * 100 + k * 50) {
                    println!("{} * 500 + {} * 100 + {} * 50", i, j, k);
                    coins.push((i, j, k))
                }
            }
        }
    }
    println!("{:?}", coins)
}
