use std::env;
use std::fs;

fn readtext() {
    let args: Vec<String> = env::args().collect();
    let file1 = &args[1];
    let file2 = &args[2];

    println!("Reading from {} and {}", file1, file2);
}
