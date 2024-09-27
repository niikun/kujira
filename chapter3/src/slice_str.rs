pub fn run() {
    //     let s = String::from("hello worls");
    //     let hello = &s[0..5];
    //     println!("{}", hello);

    //     let a: [i32; 6] = [0, 1, 2, 3, 4, 5];
    //     let slice = &a[1..3];

    //     for e in slice {
    //         println!("{}", e);
    //     }

    let v: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
    let slice = &v[3..];
    println!("{:?}", slice);
    let total_sum = sum_items(slice);
    println!("{}", total_sum);
}

fn sum_items(items: &[i32]) -> i32 {
    let mut total = 0;
    for item in items {
        total += item;
    }
    total
}
