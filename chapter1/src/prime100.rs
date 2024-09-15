fn main() {
    let mut num = 2;
    let mut primes = Vec::new();
    let mut count = 0;
    while count < 100 {
        let mut counter = 0;
        for i in 1..=num {
            if num % i == 0 {
                counter += 1;
            }
        }
        if counter == 2 {
            println!("{}", num);
            primes.push(num);
            count += 1;
        }
        num += 1;
    }
    println!("{:?}", primes);
}
