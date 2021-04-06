use crate::clear;
use crate::maze::{Maze, MazeBuilder};

pub struct Game;

impl Game {
    pub fn start() {
        clear!(all);
        let mut maze: Maze = MazeBuilder::new(32, 32).place_obstacles(150).build();
        maze.start_game_loop();
    }
}
