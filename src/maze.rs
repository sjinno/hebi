use rand::{self, Rng}; // rand = "0.8.3"
use std::collections::VecDeque;
use std::fmt;
use std::thread;
use std::time::Duration;
use termion::color::{self, LightGreen, Reset, Yellow}; // termion = "1.5.6"

#[derive(Clone, Copy, Eq, PartialEq)]
enum Square {
    Empty,
    Visited,
    Block,
    Mouse,
    Cheese,
}

impl fmt::Display for Square {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Square::Empty | Square::Visited => fmt.write_str(" "),
            Square::Block => fmt.write_str("■"),
            Square::Cheese => write!(fmt, "{}▲{}", color::Fg(Yellow), color::Fg(Reset)),
            Square::Mouse => write!(fmt, "{}●{}", color::Fg(LightGreen), color::Fg(Reset)),
        }
    }
}

struct Coord(usize, usize);

#[derive(Clone)]
pub struct Maze {
    maze: Vec<Vec<Square>>,
    m: usize,
    n: usize,
}

pub struct MazeBuilder {
    maze: Vec<Vec<Square>>,
    m: usize,
    n: usize,
}

impl MazeBuilder {
    pub fn new(m: usize, n: usize) -> Self {
        let row_bound = m + 2;
        let col_bound = n + 2;
        let maze = Self {
            maze: vec![vec![Square::Empty; col_bound]; row_bound],
            m: row_bound,
            n: col_bound,
        };
        maze.build_walls()
    }

    pub fn place_obstacles(mut self, num_of_blocks: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut count = 0;
        while count < num_of_blocks {
            let r = rng.gen_range(1..self.m);
            let c = rng.gen_range(1..self.n);
            self.maze[r][c] = Square::Block;
            count += 1;
        }
        self
    }

    fn build_walls(mut self) -> Self {
        (0..self.m).into_iter().for_each(|c| {
            self.maze[0][c] = Square::Block;
            self.maze[self.m - 1][c] = Square::Block;
        });
        (0..self.n).into_iter().for_each(|c| {
            self.maze[c][0] = Square::Block;
            self.maze[c][self.n - 1] = Square::Block;
        });
        self
    }

    pub fn build(self) -> Maze {
        Maze {
            maze: self.maze,
            m: self.m,
            n: self.n,
        }
    }
}

impl Maze {
    pub fn start_game_loop(&mut self) {
        let mouse = self.place_object(Square::Mouse);
        let mut start_info = (mouse.0, mouse.1, self.m, self.n);
        let mut snake = VecDeque::<(usize, usize)>::new();
        let mut score = 0;
        loop {
            let cheese = self.place_object(Square::Cheese);
            let path = find_paths(self.maze.clone(), start_info, Square::Cheese); // Only get the shortest path.
            draw(path, self.maze.clone(), start_info, &score, &mut snake);
            thread::sleep(Duration::from_nanos(1));
            self.maze[cheese.0][cheese.1] = Square::Empty;
            self.maze[mouse.0][mouse.1] = Square::Empty;
            start_info = (cheese.0, cheese.1, self.m, self.n);
            score += 1;
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

    for p in path.chars() {
        if snake.len() < 6 {
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

        println!("Current score: {}", score);
        for row in &mz {
            for col in row {
                print!("{} ", col);
            }
            println!();
        }

        thread::sleep(Duration::from_millis(60));
        println!("\x1B[2J\x1B[1;1H");
        snake.push_back((r, c));
    }
    None
}
