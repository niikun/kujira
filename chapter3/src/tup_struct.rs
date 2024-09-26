struct Item(String, i32);

pub fn run() {
    let banana = Item("banana".to_string(), 300);
    let apple = Item("apple".to_string(), 200);
    let mango = Item("mango".to_string(), 500);

    let items = vec![banana, apple, mango];
    let total = total_price(&items);
    println!("total:{}", total);
}

fn print_item(item: &Item) {
    println!("{}:{}", item.0, item.1);
}

fn total_price(items: &Vec<Item>) -> i32 {
    let mut total = 0;
    for item in items {
        print_item(item);
        total += item.1;
    }
    total
}
