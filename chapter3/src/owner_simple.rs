pub fn run() {
    let g1 = String::from("niikun");
    let mut g2 = gen_msg();
    add_quote(&mut g2);
    println!("{},{}", g1, g2);
}

fn gen_msg() -> String {
    let msg = String::from("hello func");
    msg
}

fn add_quote(s: &mut String) {
    s.insert(0, '『');
    s.push('』');
}
