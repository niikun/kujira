use std::collections::HashMap;

const VALUES: &str = "c,c,a,a,a,b,c,c,b,b,b,c,b,c,b,a";

pub fn run() {
    let mut evals = HashMap::new();
    evals.insert("a", 0);
    evals.insert("b", 0);
    evals.insert("c", 0);

    for val in VALUES.split(",") {
        // println!("{}", val);
        evals.insert(val, evals[val] + 1);
    }
    println!("{:?}", evals);
}
