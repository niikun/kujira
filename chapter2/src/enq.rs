use std::collections::HashMap;
use std::io::stdin;

fn get_data() -> Vec<String> {
    let mut values = String::new();
    // let mut datas: Vec<String> = Vec::new();
    println!("データを入力してください");
    stdin().read_line(&mut values).expect("入力エラー");
    let datas = values
        .trim()
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<String>>();
    // println!("{:?}", values);
    // println!("{:?}", datas);
    datas
}

fn get_counts(datas: Vec<String>) {
    let mut eval = HashMap::new();
    eval.insert("a", 0);
    eval.insert("b", 0);
    eval.insert("c", 0);
    for data in datas {
        match data.as_str() {
            "a" => *eval.entry("a").or_insert(0) += 1,
            "b" => *eval.entry("b").or_insert(0) += 1,
            "c" => *eval.entry("c").or_insert(0) += 1,
            _ => println!("other"),
        }
    }
    println!("{:?}", eval);
}

pub fn run() {
    let evals = get_data();
    get_counts(evals);
}
