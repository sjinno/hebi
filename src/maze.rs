use rand::{self, Rng};
use std::collections::VecDeque;
use std::thread;
use std::time::Duration;
use termion::color::{self, LightGreen, Reset, Yellow};

struct Coord(usize, usize);

#[derive(Clone)]
pub struct Maze {
    maze: Vec<Vec<String>>,
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
        let num_of_blocks = 150;

        let mut count = 0;
        while count < num_of_blocks {
            let r = rng.gen_range(1..row_bound);
            let c = rng.gen_range(1..col_bound);
            maze[r][c] = block.clone();
            count += 1;
        }

        // Build walls.
        (0..col_bound).into_iter().for_each(|c| {
            maze[0][c] = block.clone();
            maze[row_bound - 1][c] = block.clone();
        });
        (0..row_bound).into_iter().for_each(|c| {
            maze[c][0] = block.clone();
            maze[c][col_bound - 1] = block.clone();
        });

        Self {
            maze,
            m: row_bound,
            n: col_bound,
        }
    }

    pub fn start_game_loop(&mut self) {
        let mouse = self.place_object(format!("{}●{}", color::Fg(LightGreen), color::Fg(Reset)));
        let mut start_info = (mouse.0, mouse.1, self.m, self.n);

        loop {
            let cheese = self.place_object(format!("{}▲{}", color::Fg(Yellow), color::Fg(Reset)));
            let path = find_paths(
                self.maze.clone(),
                start_info,
                format!("{}▲{}", color::Fg(Yellow), color::Fg(Reset)),
            )[0]
            .clone(); // Oonly get the shortest path.

            draw(path, self.maze.clone(), start_info);

            thread::sleep(Duration::from_nanos(1));
            self.maze[cheese.0][cheese.1] = String::from(" ");
            self.maze[mouse.0][mouse.1] = String::from(" ");
            start_info = (cheese.0, cheese.1, self.m, self.n);
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

fn find_paths(
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
        let (row, col) = (r, c);
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

fn draw(path: String, maze: Vec<Vec<String>>, start_info: (usize, usize, usize, usize)) {
    let mut m = maze;
    let (mut r, mut c, _, _) = start_info;
    let mut snake = VecDeque::<(usize, usize)>::new();
    for p in path.chars() {
        if snake.len() < 5 {
            snake.push_back((r, c));
        } else {
            let (tail_r, tail_c) = snake.pop_front().unwrap();
            m[tail_r][tail_c] = String::from(" ");
            snake.push_back((r, c));
        }
        match p {
            'U' => r -= 1,
            'D' => r += 1,
            'L' => c -= 1,
            'R' => c += 1,
            _ => {}
        }
        m[r][c] = format!("{}●{}", color::Fg(LightGreen), color::Fg(Reset));
        for row in &m {
            for col in row {
                print!("{} ", col);
            }
            println!();
        }
        thread::sleep(Duration::from_millis(60));
        println!("\x1B[2J\x1B[1;1H");
    }
}
