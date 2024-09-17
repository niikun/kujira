// use std::fs::{self, File};
// use std::io::{BufWriter, Write};

// pub fn run() {
//     let filename = "rec_sum.txt";
//     let mut fp = File::create(filename).unwrap();
//     let mut writer = BufWriter::new(fp);

//     let mut nums: Vec<i32> = vec![0, 1];
//     for i in 2..10 {
//         let num = nums[i - 2] + nums[i - 1];
//         nums.push(num);
//     }
//     for num in &nums {
//         let line = format!("{}\n", num);
//         let bytes = line.as_bytes();
//         writer.write(bytes).unwrap();
//     }
//     writer.flush().unwrap();

//     let s = fs::read_to_string(filename).unwrap();
//     println!("{}", s);
// }

fn sum(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }
    sum(n - 1) + n
}

pub fn run() {
    let num = sum(10);
    println!("{}", num);
}
