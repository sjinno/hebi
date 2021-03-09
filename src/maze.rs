// use perf_event::Builder;
use rand::{self, Rng}; // rand = "0.8.3"
use std::collections::VecDeque;
use std::fmt;
use std::thread;
use std::time::Duration;
use termion::color::{self, LightGreen, Reset, Yellow}; // termion = "1.5.6" // perf-event = "0.4"

struct Coord(usize, usize);

#[derive(Clone)]
pub struct Maze {
    maze: Vec<Vec<Square>>,
    m: usize,
    n: usize,
}

/// Contents of a square in the maze.
///
/// In practice, Rust will assign these the the numbers 0..4, and represent
/// `Square` values as single bytes. `Copy` means that you don't need to use
/// `clone`: they're sort of always cloned. `Copy` is convenient, but it's only
/// permitted for simple types like this; you can't use it for something more
/// complex, like `Maze`.
#[derive(Clone, Copy, Eq, PartialEq)]
enum Square {
    Empty,
    Visited,
    Block,
    Mouse,
    Cheese,
}

/// Tell `println!` how `{}` should display `Square` values.
impl fmt::Display for Square {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Square::Empty | Square::Visited => fmt.write_str(" "),
            Square::Block => fmt.write_str("■"),
            Square::Mouse => write!(fmt, "{}●{}", color::Fg(LightGreen), color::Fg(Reset)),
            Square::Cheese => write!(fmt, "{}▲{}", color::Fg(Yellow), color::Fg(Reset)),
        }
    }
}

impl Maze {
    pub fn new(m: usize, n: usize) -> Self {
        let row_bound = m + 2;
        let col_bound = n + 2;

        let mut maze = vec![vec![Square::Empty; col_bound]; row_bound];
        let mut rng = rand::thread_rng();

        let num_of_blocks = 150;

        let mut count = 0;
        while count < num_of_blocks {
            let r = rng.gen_range(1..row_bound);
            let c = rng.gen_range(1..col_bound);
            maze[r][c] = Square::Block;
            count += 1;
        }

        // Build walls.
        (0..col_bound).into_iter().for_each(|c| {
            maze[0][c] = Square::Block;
            maze[row_bound - 1][c] = Square::Block;
        });
        (0..row_bound).into_iter().for_each(|c| {
            maze[c][0] = Square::Block;
            maze[c][col_bound - 1] = Square::Block;
        });

        Self {
            maze,
            m: row_bound,
            n: col_bound,
        }
    }

    pub fn start_game_loop(&mut self) {
        let mouse = self.place_object(Square::Mouse);
        let mut start_info = (mouse.0, mouse.1, self.m, self.n);
        let mut snake = VecDeque::<(usize, usize)>::new();

        // let mut counter = Builder::new()
        //     .build()
        //     .expect("failed to create instruction counter");

        let mut score = 0;

        loop {
            let cheese = self.place_object(Square::Cheese);

            // counter.reset().expect("reset counter");
            // counter.enable().expect("enable counter");

            let path = find_paths(self.maze.clone(), start_info, Square::Cheese); // Only get the shortest path.
            score += 1;

            // let instructions = counter
            //     .read()
            //     .expect("error reading find_paths instruction count");
            // counter.disable().expect("disable counter");

            // // NEW FEATURE +++++
            let point = draw(path, self.maze.clone(), start_info, &score, &mut snake);
            // if let Some(_) = point {
            //     score += 1;
            // }

            // if score == 10 {
            //     println!("You win!");
            //     for row in &self.maze {
            //         for col in row {
            //             print!("{} ", col);
            //         }
            //         println!();
            //     }
            //     break;
            // }
            // // +++++++++++++++++

            thread::sleep(Duration::from_nanos(1));
            self.maze[cheese.0][cheese.1] = Square::Empty;
            self.maze[mouse.0][mouse.1] = Square::Empty;
            start_info = (cheese.0, cheese.1, self.m, self.n);
        }
    }

    fn place_object(&mut self, obj: Square) -> Coord {
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(1..self.m - 1);
        let c = rng.gen_range(1..self.n - 1);
        self.maze[r][c] = obj;
        Coord(r, c)
    }
}

fn find_paths(
    mut maze: Vec<Vec<Square>>,
    start_info: (usize, usize, usize, usize),
    target: Square,
) -> String {
    let (row, col, row_len, col_len) = start_info;
    let mut pos_queue = VecDeque::<(usize, usize)>::new();
    let mut path_queue = VecDeque::<String>::new();
    pos_queue.push_back((row, col));
    path_queue.push_back("S".to_string());

    while let Some(pos) = pos_queue.pop_front() {
        let (r, c) = pos;

        let path = path_queue.pop_front().unwrap();
        let (row, col) = (r, c);
        if row > 0 {
            let up = row - 1;
            let path = update_queues(
                &mut maze,
                &mut pos_queue,
                &mut path_queue,
                path.clone(),
                'U',
                up,
                col,
                target.clone(),
            );
            if let Some(p) = path {
                return p;
            }
        }
        if row < row_len - 1 {
            let down = row + 1;
            let path = update_queues(
                &mut maze,
                &mut pos_queue,
                &mut path_queue,
                path.clone(),
                'D',
                down,
                col,
                target.clone(),
            );
            if let Some(p) = path {
                return p;
            }
        }
        if col > 0 {
            let left = col - 1;
            let path = update_queues(
                &mut maze,
                &mut pos_queue,
                &mut path_queue,
                path.clone(),
                'L',
                row,
                left,
                target.clone(),
            );
            if let Some(p) = path {
                return p;
            }
        }
        if col < col_len - 1 {
            let right = col + 1;
            let path = update_queues(
                &mut maze,
                &mut pos_queue,
                &mut path_queue,
                path.clone(),
                'R',
                row,
                right,
                target.clone(),
            );
            if let Some(p) = path {
                return p;
            }
        }
    }

    String::new()
}

fn update_queues(
    maze: &mut Vec<Vec<Square>>,
    pos_queue: &mut VecDeque<(usize, usize)>,
    path_queue: &mut VecDeque<String>,
    path: String,
    direction: char,
    row: usize,
    col: usize,
    target: Square,
) -> Option<String> {
    let p = maze[row][col];
    if p == target {
        return Some(format!("{}{}", path, direction));
    } else if p == Square::Empty {
        maze[row][col] = Square::Visited;
        pos_queue.push_back((row, col));
        path_queue.push_back(format!("{}{}", path, direction));
    }
    None
}

fn draw(
    path: String,
    maze: Vec<Vec<Square>>,
    start_info: (usize, usize, usize, usize),
    score: &usize,
    snake: &mut VecDeque<(usize, usize)>,
) -> Option<()> {
    let mut mz = maze;
    let (mut r, mut c, _, _) = start_info;
    snake.push_back((r, c));

    for p in path.chars() {
        if snake.len() < *score {
            snake.push_back((r, c));
            let (tail_r, tail_c) = snake.pop_front().unwrap();
            mz[tail_r][tail_c] = Square::Empty;
        } else {
            let (tail_r, tail_c) = snake.pop_front().unwrap();
            mz[tail_r][tail_c] = Square::Empty;
        }

        match p {
            'U' => r -= 1,
            'D' => r += 1,
            'L' => c -= 1,
            'R' => c += 1,
            _ => {}
        }
        mz[r][c] = Square::Mouse;

        // NEW FEATURE STARTS HERE
        // match (r, c) {
        //     (_, 1..=10) | (1..=10, _) => return Some(()),
        //     _ => (),
        // }
        // println!("Current score: {}", score);
        // +++++++++++++++++++++++
        println!("Current score: {}", score);
        for row in &mz {
            for col in row {
                print!("{} ", col);
            }
            println!();
        }

        // println!("instructions to find paths: {}", instructions);

        thread::sleep(Duration::from_millis(60));
        println!("\x1B[2J\x1B[1;1H");
        snake.push_back((r, c));
    }
    None
}
