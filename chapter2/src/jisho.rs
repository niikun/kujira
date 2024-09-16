use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let text: &str = &args[1].trim();
    println!("Searching for {}", text);
    let dicfile = match File::open("ejdict-hand-utf8.txt") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    let reader = BufReader::new(dicfile);
    let mut found = false;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains(text) {
            println!("{}", line);
            found = true;
        }
    }
    if found == false {
        println!("No match found");
    }
}
