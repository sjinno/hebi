use std::fmt::{self, Write};

use rand::{self, Rng}; // rand = "0.8.3"
use termion::color::{self, LightGreen, Reset, Yellow}; // termion = "1.5.6"

use crate::color;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Square {
    Empty,
    Visited,
    Block,
    Snake,
    Bait,
}

impl fmt::Display for Square {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Square::Empty | Square::Visited => fmt.write_char(' '),
            Square::Block => fmt.write_char('■'),
            Square::Bait => write!(fmt, "{}", color!(bait)),
            Square::Snake => write!(fmt, "{}", color!(snake)),
        }
    }
}

pub struct Coord(pub usize, pub usize);

#[derive(Clone)]
pub struct Maze {
    pub maze: Vec<Vec<Square>>,
    pub m: usize,
    pub n: usize,
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
    pub fn place_object(&mut self, obj: Square) -> Coord {
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(1..self.m - 1);
        let c = rng.gen_range(1..self.n - 1);
        self.maze[r][c] = obj;
        Coord(r, c)
    }
}
