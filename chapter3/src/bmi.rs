use std::io;

struct Body {
    name: String,
    height: f64,
    weight: f64,
}

impl Body {
    fn new(name: String, height: f64, weight: f64) -> Body {
        Body {
            name,
            height,
            weight,
        }
    }

    fn cal_bmi(&self) -> f64 {
        let bmi = self.weight / self.height.powf(2.0);
        bmi
    }

    fn judge(&self) -> &str {
        let bmi = self.cal_bmi();
        match bmi {
            bmi if bmi < 18.5 => "痩せ型",
            bmi if bmi < 25.0 => "普通",
            bmi if bmi < 30.0 => "肥満(1度)",
            bmi if bmi < 35.0 => "肥満(2度)",
            _ => "肥満(3度)",
        }
    }
}

pub fn run() {
    // 名前を取得
    println!("名前:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim().to_string(); // 改行を除去し、Stringに変換

    // 身長を取得
    println!("身長 (m):");
    let mut height_input = String::new();
    io::stdin()
        .read_line(&mut height_input)
        .expect("Failed to read line");
    let height: f64 = height_input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    // 体重を取得
    println!("体重 (kg):");
    let mut weight_input = String::new();
    io::stdin()
        .read_line(&mut weight_input)
        .expect("Failed to read line");
    let weight: f64 = weight_input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    let body = Body::new(name, height, weight);
    let bmi = body.cal_bmi();
    println!("BMI of {} : {:.2} =>{:?}", body.name, bmi, body.judge());
}
