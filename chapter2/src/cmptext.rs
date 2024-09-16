use std::env;
use std::fs;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let file1 = &args[1];
    let file2 = &args[2];
    // println!("Reading from {} and {}", file1, file2);
    let text1 = fs::read_to_string(&file1).expect("Unable to read file");
    let text2 = fs::read_to_string(&file2).expect("Unable to read file");
    println!("{} \nand \n{}", text1, text2);
    if text1 == text2 {
        println!("The files are the same");
    } else {
        println!("The files are different");
    }
}
