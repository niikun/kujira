use std::{env, path};

pub fn run() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: findfile <filename>");
    }
    let target_dir = &args[1];
    let keyword = &args[2];
    println!("Searching for {} in {}", keyword, target_dir);
    let target = path::PathBuf::from(target_dir);
    findfile(&target, keyword);
}

fn findfile(target: &path::PathBuf, keyword: &str) {
    let files = target.read_dir().expect("そんなディレクトリはありません");
    for dir_entry in files {
        let path = dir_entry.unwrap().path();
        if path.is_dir() {
            findfile(&path, keyword);
            continue;
        }
        let fname = path.file_name().unwrap().to_string_lossy();
        if fname.contains(keyword) {
            println!("{}", path.to_string_lossy());
        }
    }
}
