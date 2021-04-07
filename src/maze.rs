use rand::{self, Rng}; // rand = "0.8.3"

use crate::square::Square;

#[derive(Clone, Copy)]
pub struct Coord(pub usize, pub usize);

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
        let row_bound = m + 2;
        let col_bound = n + 2;
        let maze = Self {
            maze: vec![vec![Square::Empty; col_bound]; row_bound],
            field: Field {
                width: row_bound,
                height: col_bound,
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
        (0..self.field.height).into_iter().for_each(|c| {
            self.maze[0][c] = Square::Block;
            self.maze[self.field.height - 1][c] = Square::Block;
        });
        (0..self.field.width).into_iter().for_each(|c| {
            self.maze[c][0] = Square::Block;
            self.maze[c][self.field.width - 1] = Square::Block;
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

impl Maze {
    pub fn place_object(&mut self, obj: Square) -> Coord {
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(1..self.field.height - 1);
        let c = rng.gen_range(1..self.field.width - 1);
        self.maze[r][c] = obj;
        Coord(r, c)
    }
}
