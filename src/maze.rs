use rand::{self, Rng};
use std::collections::VecDeque;
use termion::color::{self, Green, Magenta, Reset, Yellow};

#[derive(Clone)]
pub struct Maze {
    pub maze: Vec<Vec<String>>,
    m: usize,
    n: usize,
}

impl Maze {
    pub fn new(m: usize, n: usize) -> Self {
        let row_bound = m + 2;
        let col_bound = n + 2;

        let mut maze = vec![vec![String::from(" "); col_bound]; row_bound];
        let mut rng = rand::thread_rng();

        let block = String::from("■");
        let num_of_blocks = 128;

        let mut count = 0;
        while count < num_of_blocks {
            let r = rng.gen_range(1..row_bound);
            let c = rng.gen_range(1..col_bound);
            maze[r][c] = block.clone();
            count += 1;
        }

        Maze::build_walls(&mut maze, row_bound, col_bound, block);

        Self {
            maze,
            m: row_bound,
            n: col_bound,
        }
    }

    fn build_walls(maze: &mut Vec<Vec<String>>, row_bound: usize, col_bound: usize, block: String) {
        (0..col_bound).into_iter().for_each(|c| {
            maze[0][c] = block.clone();
            maze[row_bound - 1][c] = block.clone();
        });
        (0..row_bound).into_iter().for_each(|c| {
            maze[c][0] = block.clone();
            maze[c][col_bound - 1] = block.clone();
        });
    }

    pub fn draw_maze(&mut self) {
        let mut cheese = self.place_object(format!("{}▲{}", color::Fg(Yellow), color::Fg(Reset)));
        let mut mouse = self.place_object(format!("{}●{}", color::Fg(Magenta), color::Fg(Reset)));

        for row in self.maze.iter() {
            for col in row {
                print!("{} ", col);
            }
            println!();
        }
    }

    fn place_object(&mut self, obj: String) -> Coord {
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(1..self.m - 1);
        let c = rng.gen_range(1..self.n - 1);
        self.maze[r][c] = obj;

        Coord(r, c)
    }
}

struct Coord(usize, usize);

// PATH FINDING METHOD
fn update_queues(
    maze: &mut Vec<Vec<String>>,
    pos_queue: &mut VecDeque<(usize, usize)>,
    path_queue: &mut VecDeque<String>,
    paths: &mut Vec<String>,
    path: String,
    direction: char,
    row: usize,
    col: usize,
    target: String,
) {
    let p = &maze[row][col];
    if p == &target {
        paths.push(format!("{}{}", path, direction));
    } else if p == " " {
        maze[row][col] = format!("{} {}", color::Fg(Yellow), color::Fg(Reset));
        pos_queue.push_back((row, col));
        path_queue.push_back(format!("{}{}", path, direction));
    }
}

pub fn find_paths(
    mut maze: Vec<Vec<String>>,
    start_info: (usize, usize, usize, usize),
    target: String,
) -> Vec<String> {
    let (row, col, row_len, col_len) = start_info;
    let mut pos_queue = VecDeque::<(usize, usize)>::new();
    let mut path_queue = VecDeque::<String>::new();
    pos_queue.push_back((row, col));
    path_queue.push_back("S".to_string());

    let mut paths = vec![];
    while let Some(pos) = pos_queue.pop_front() {
        // println!("{:?}", pos);
        let (r, c) = pos;
        let path = path_queue.pop_front().unwrap();
        let (row, col) = (r.clone(), c.clone());
        if row > 0 {
            let up = row - 1;
            update_queues(
                &mut maze,
                &mut pos_queue,
                &mut path_queue,
                &mut paths,
                path.clone(),
                'U',
                up,
                col,
                target.clone(),
            );
        }
        if row < row_len - 1 {
            let down = row + 1;
            update_queues(
                &mut maze,
                &mut pos_queue,
                &mut path_queue,
                &mut paths,
                path.clone(),
                'D',
                down,
                col,
                target.clone(),
            );
        }
        if col > 0 {
            let left = col - 1;
            update_queues(
                &mut maze,
                &mut pos_queue,
                &mut path_queue,
                &mut paths,
                path.clone(),
                'L',
                row,
                left,
                target.clone(),
            );
        }
        if col < col_len - 1 {
            let right = col + 1;
            update_queues(
                &mut maze,
                &mut pos_queue,
                &mut path_queue,
                &mut paths,
                path.clone(),
                'R',
                row,
                right,
                target.clone(),
            );
        }
    }

    paths
}

// pub fn draw(path: String, maze: Vec<Vec<String>>, start_info: (usize, usize, usize, usize)) {
//     let mut m = maze.clone();
//     let (mut r, mut c, _, _) = start_info;
//     for p in path.chars() {
//         match p {
//             'U' => r -= 1,
//             'D' => r += 1,
//             'L' => c -= 1,
//             'R' => c += 1,
//             _ => {}
//         }
//         m[r][c] = "■".to_string();
//     }

//     for row in m {
//         for col in row {
//             print!("{} ", col);
//         }
//         println!();
//     }
// }
