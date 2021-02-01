use rand::{self, Rng};
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
