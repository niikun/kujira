use std::io::stdin;

pub fn run() {
    let mut height;
    loop {
        println!("身長を入力してください");

        height = input_f(0.0);
        if height > 0.0 {
            break;
        }
    }
}

fn input_str() -> String {
    let mut s = String::new();
    stdin().read_line(&mut s).expect("入力エラー");
    s.trim_end().to_string()
}

fn input_f(def: f64) -> f64 {
    let s = input_str();

    match s.trim().parse() {
        Ok(v) => v,
        Err(_) => def,
    }
}
