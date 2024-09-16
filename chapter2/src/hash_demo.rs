use std::{collections::HashMap, hash::Hash};

fn greet_map(id: usize, name: String) -> HashMap<usize, String> {
    let mut map: HashMap<usize, String> = HashMap::new();
    map.insert(id, name);
    map
}

pub fn run() {
    let id = 1;
    let name = "niikun".to_string();
    println!("{:?}", greet_map(id, name));
}
