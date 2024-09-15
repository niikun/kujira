fn is_prime(n: u32) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

fn get_prime(primes: &mut [u32; 100]) -> [u32; 100] {
    let mut i = 2;
    let mut count = 0;
    while count < 100 {
        if is_prime(i) {
            primes[count] = i;
            count += 1;
        }
        i += 1;
    }
    *primes
}

fn main() {
    let mut primes = [0; 100];
    get_prime(&mut primes);
    println!("{:?}", primes);
}
