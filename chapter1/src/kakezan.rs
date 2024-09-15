use std::io;

fn circle_area(a: f64) -> f64 {
    (a * a) * std::f64::consts::PI
}

fn main() {
    println!("Input a:");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Failed to read line");
    let a_float: f64 = a.trim().parse().expect("Please type a number");

    println!("{:.4}", circle_area(a_float));
}
