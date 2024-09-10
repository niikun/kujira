use rand::Rng;

fn set_card() -> Vec<[u32; 5]> {
    let mut rng = rand::thread_rng();
    let mut card = vec![[0; 5]; 5];
    for i in 0..5 {
        for j in 0..5 {
            if i == 2 && j == 2 {
                continue;
            }
            let mut num = rng.gen_range(1..99);
            card[i][j] = num;
        }
    }
    card
}

fn print_card(card: &Vec<[u32; 5]>) {
    for i in 0..5 {
        for j in 0..5 {
            print!("{:02} ", card[i][j]);
        }
        println!();
    }
}

pub fn run() {
    let card = set_card();
    println!("Bingo! ");
    print_card(&card);
}
