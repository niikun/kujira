use std::fs;
use std::io;
use std::path::Path;

pub fn run() -> io::Result<()> {
    // 相対パスを指定
    let path = Path::new("../chapter1/");
    let files = fs::read_dir(path)?;

    for ent in files {
        let entry = ent?;
        let path = entry.path();
        let fname = path.to_string_lossy();
        println!("{}", fname);
    }
    Ok(())
}
