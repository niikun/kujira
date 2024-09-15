use std::env;
use std::fs;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() < 2 {
        println!("Please provide a file name");
        return;
    }
    for (i, file) in args.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let text = fs::read_to_string(file).expect("Failed to read file");
        println!("{}", text);
    }
    // let filename = &args[1];
    // let text = fs::read_to_string(filename).expect("Failed to read file");
    // println!("{}", text);
}
