use std::io::{self, stdin};

pub fn run() {
    let mut weight = String::new();
    let mut height = String::new();

    println!("enter your weight in kg: ");
    stdin().read_line(&mut weight).expect("Failed to read line");
    let weight: f32 = weight.trim().parse().expect("Please type a number!");

    println!("enter your height in m: ");
    stdin().read_line(&mut height).expect("Failed to read line");
    let height: f32 = height.trim().parse().expect("Please type a number!");
    let bmi = weight / height.powf(2.0);

    println!(
        "weight: {:.2} kg ,height: {:.2}m ,BMI :{:.2}",
        weight, height, bmi
    );
    if bmi < 18.5 {
        println!("低体重")
    } else if bmi < 24.9 {
        println!("普通")
    } else if bmi < 30.0 {
        println!("肥満１")
    } else if bmi < 35.0 {
        println!("肥満２")
    } else if bmi < 40.0 {
        println!("肥満３")
    } else {
        println!("肥満４")
    }
}
