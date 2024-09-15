use std::env;

pub fn run() {
    let mut args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    println!(
        "{}",
        args[1].parse::<f64>().unwrap() + args[2].parse::<f64>().unwrap()
    );
}

// pub fn run() {
//     let mut num = env::args();
//     println!("{:?}", num);
//     for (i, s) in num.enumerate() {
//         if i == 0 {
//             continue;
//         }
//         let mut s_f: f64 = s.parse().unwrap();
//         println!("{},{}", i, s_f);
//     }
// }

// pub fn run() {
//     let args = std::env::args();
//     let mut total = 0.0;
//     for (i, s) in args.enumerate() {
//         // println!("i:{} , s:{}", i, s);
//         if i >= 1 {
//             let num: f64 = match s.parse() {
//                 Ok(v) => v,
//                 Err(_) => 0.0,
//             };
//             total += num;
//         }
//     }
//     println!("Total: {}", total);
// }
