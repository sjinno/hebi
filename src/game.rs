use crate::clear;
use crate::maze::{Maze, MazeBuilder};
use crate::snake::Snake;

pub struct Game;

impl Game {
    pub fn start() {
        clear!(all);
        let mut maze: Maze = MazeBuilder::new(32, 32).place_obstacles(150).build();
        maze.activate_snake();
    }
}
