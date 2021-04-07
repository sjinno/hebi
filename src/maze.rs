use crate::square::Square;
use rand::{self, Rng}; // rand = "0.8.3"
use std::collections::VecDeque;
use std::thread;
use std::time::Duration;

use crate::clear;

#[derive(Clone, Copy)]
pub struct Field {
    pub height: usize,
    pub width: usize,
}

#[derive(Clone)]
pub struct Maze {
    pub maze: Vec<Vec<Square>>,
    pub field: Field,
}

pub struct MazeBuilder {
    maze: Vec<Vec<Square>>,
    field: Field,
}

impl MazeBuilder {
    pub fn new(m: usize, n: usize) -> Self {
        let maze = Self {
            maze: vec![vec![Square::Empty; n + 2]; m + 2],
            field: Field {
                height: m + 2,
                width: n + 2,
            },
        };
        maze.build_walls()
    }

    pub fn place_obstacles(mut self, num_of_blocks: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut count = 0;
        while count < num_of_blocks {
            let r = rng.gen_range(1..self.field.height);
            let c = rng.gen_range(1..self.field.width);
            self.maze[r][c] = Square::Block;
            count += 1;
        }
        self
    }

    fn build_walls(mut self) -> Self {
        (0..self.field.height).into_iter().for_each(|r| {
            self.maze[r][0] = Square::Block;
            self.maze[r][self.field.width - 1] = Square::Block;
        });
        (0..self.field.width).into_iter().for_each(|c| {
            self.maze[0][c] = Square::Block;
            self.maze[self.field.height - 1][c] = Square::Block;
        });
        self
    }

    pub fn build(self) -> Maze {
        Maze {
            maze: self.maze,
            field: self.field,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Coord(pub usize, pub usize);

impl Maze {
    pub fn place_object(&mut self, obj: Square) -> Coord {
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(1..self.field.height - 1);
        let c = rng.gen_range(1..self.field.width - 1);
        self.maze[r][c] = obj;
        Coord(r, c)
    }

    pub fn draw(
        path: String,
        maze: Vec<Vec<Square>>,
        start_info: (Coord, Field),
        score: &usize,
        snake_size: &mut VecDeque<(usize, usize)>,
    ) {
        let mut mz = maze;
        let (mut coord, _) = start_info;

        for p in path.chars() {
            if snake_size.len() < 6 {
                snake_size.push_back((coord.0, coord.1));
                let (tail_r, tail_c) = snake_size.pop_front().unwrap();
                mz[tail_r][tail_c] = Square::Empty;
            } else {
                let (tail_r, tail_c) = snake_size.pop_front().unwrap();
                mz[tail_r][tail_c] = Square::Empty;
            }

            match p {
                'U' => coord.0 -= 1,
                'D' => coord.0 += 1,
                'L' => coord.1 -= 1,
                'R' => coord.1 += 1,
                _ => {}
            }
            mz[coord.0][coord.1] = Square::Snake;

            println!("Current score: {}", score);
            for row in &mz {
                for col in row {
                    print!("{} ", col);
                }
                println!();
            }

            thread::sleep(Duration::from_millis(60));
            clear!(all);
            snake_size.push_back((coord.0, coord.1));
        }
    }
}
