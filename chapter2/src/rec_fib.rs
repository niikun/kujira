fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}

pub fn run() {
    for i in 0..=10 {
        println!("fib({}) = {}", i, fib(i));
    }
}
