use std::env;
use std::fs;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let mut total = 0;

    for (i, f) in args.iter().enumerate() {
        if i == 0 {
            // 最初の引数はプログラムのパスなのでスキップ
            continue;
        } else {
            // ファイルの内容を読み込む
            let text = fs::read_to_string(f).expect("Failed to read file");

            // 読み込んだテキストをデバッグ用に表示
            // println!("File content: '{}'", text);
            let text = text.split("\n").collect::<Vec<&str>>();
            for line in text.iter() {
                // println!("Line: '{}'", line);
                // 空白や改行をトリムして数値に変換
                match line.trim().parse::<i32>() {
                    Ok(num) => {
                        total += num;
                    }
                    Err(_) => {
                        println!("Warning: '{}' is not a valid number", line);
                    }
                }
            }
        }
    }

    println!("Total: {}", total);
}
