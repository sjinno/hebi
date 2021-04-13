use crate::clear;
use crate::maze::{Maze, MazeBuilder};
use crate::snake::Snake;

pub struct Game;

impl Game {
    pub fn start() {
        // Clears the entire buffer and
        // places the curson at the very beginning.
        clear!(all);
        // Build 32x32 square maze with 150 randomly placed obstacles.
        let mut maze: Maze = MazeBuilder::new(32, 32).place_obstacles(144).build();
        maze.activate_snake();
    }
}
