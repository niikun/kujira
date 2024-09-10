use rand::seq::SliceRandom;

pub fn run() {
    let mut nums = Vec::new();
    for i in 1..=75 {
        nums.push(i)
    }

    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng);

    for y in 0..5 {
        for x in 0..5 {
            if y == 2 && x == 2 {
                print!(" * ,");
            } else {
                print!("{:3},", nums[y * 5 + x]);
            }
        }
        println!("");
    }

    // println!("{:?}", nums);
}
