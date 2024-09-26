pub fn run() {
    let g1 = ("banana", 300);
    let g2 = ("apple", 200);

    let total = g1.1 + g2.1;
    print_item(&g1);
    print_item(&g2);
    println!("total:{}", total);
}

fn print_item(&item: &(&str, i32)) {
    println!("{}:{}", item.0, item.1);
}
