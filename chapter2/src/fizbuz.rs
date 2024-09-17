use std::fs::{self, File};
use std::io::{BufWriter, Write};

pub fn run() {
    let filename = "fizzbuzz.txt";

    let fp = File::create(filename).unwrap();
    let mut writer = BufWriter::new(fp);

    for i in 1..100 {
        let mut line = format!("{}\n", i);
        if (i % 3 == 0) && (i % 5 == 0) {
            line = String::from("fizzbuzz\n");
        } else if i % 3 == 0 {
            line = String::from("fizz\n");
        } else if i % 5 == 0 {
            line = String::from("buzz\n");
        }
        let bytes = line.as_bytes();
        writer.write(bytes).unwrap();
    }
    writer.flush().unwrap();

    let s = fs::read_to_string(filename).unwrap();
    println!("{}", s);
}
