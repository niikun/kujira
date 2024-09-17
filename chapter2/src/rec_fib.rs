fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}

pub fn run() {
    let num = fib(5);
    println!("{}", num);
}
