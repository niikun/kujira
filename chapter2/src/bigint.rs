use num_bigint::BigInt;

pub fn run() {
    let a = BigInt::from(1234);
    println!("{}", a.pow(2)); // 1522756
    println!("{}", a.pow(5678));
}
