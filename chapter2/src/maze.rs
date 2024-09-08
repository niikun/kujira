use rand::Rng;

pub fn run() {
    let mut maze: Vec<Vec<usize>> = vec![vec![0; N]; N];
    const N: usize = 25;
    for i in 0..N {
        for j in 0..N {
            if i == 0 || i == N - 1 || j == 0 || j == N - 1 {
                maze[i][j] = 1;
            }
        }
    }
    let mut rng = rand::thread_rng();

    for i in 1..N - 1 {
        for j in 1..N - 1 {
            maze[i][j] = rng.gen_range(0..=1);
        }
    }
    for i in 0..N {
        for j in 0..N {
            if maze[i][j] == 0 {
                print!("  ");
            } else {
                print!("ZZ");
            }
        }
        println!();
    }
    println!("{:?}", maze);
}
