use rand::Rng;

const MAP_N: usize = 25;

fn initialize_maze(maze: &mut [[usize; MAP_N]; MAP_N]) {
    // let mut rng = rand::thread_rng();

    for i in 0..MAP_N {
        for j in 0..MAP_N {
            if i == 0 || i == MAP_N - 1 || j == 0 || j == MAP_N - 1 {
                maze[i][j] = 1;
            }
        }
    }
}

fn print_maze(maze: &[[usize; MAP_N]; MAP_N]) {
    for i in 0..MAP_N {
        for j in 0..MAP_N {
            if maze[i][j] == 0 {
                print!("  ");
            } else {
                print!("ZZ");
            }
        }
        println!();
    }
}

fn set_maze(maze: &mut [[usize; MAP_N]; MAP_N]) {
    let mut rng = rand::thread_rng();
    for i in 2..MAP_N - 2 {
        for j in 2..MAP_N - 2 {
            if i % 2 == 1 || j % 2 == 1 {
                continue;
            }
            maze[i][j] = 1;
            let r = rng.gen_range(0..=3);
            match r {
                0 => maze[i - 1][j] = 1,
                1 => maze[i + 1][j] = 1,
                2 => maze[i][j - 1] = 1,
                3 => maze[i][j + 1] = 1,
                _ => {}
            }
        }
    }
}

pub fn run() {
    let mut maze = [[0; MAP_N]; MAP_N];
    initialize_maze(&mut maze);
    set_maze(&mut maze);
    print_maze(&mut maze);
}
