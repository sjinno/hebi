use std::collections::VecDeque;
use std::thread;
use std::time::Duration;

use crate::maze::Maze;
use crate::path_finder::*;
use crate::square::Square;

pub trait Snake {
    fn activate_snake(&mut self);
}

impl Snake for Maze {
    fn activate_snake(&mut self) {
        let snake = self.place_object(Square::Snake);
        let mut start_info = (snake, self.field);
        let mut snake_size = VecDeque::<(usize, usize)>::new();
        let mut score = 0;
        loop {
            let cheese = self.place_object(Square::Bait);
            let path = find_paths(self.maze.clone(), start_info, Square::Bait); // Only get the shortest path.
            Self::draw(path, self.maze.clone(), start_info, &score, &mut snake_size);
            thread::sleep(Duration::from_nanos(1));
            self.maze[cheese.0][cheese.1] = Square::Empty;
            self.maze[snake.0][snake.1] = Square::Empty;
            start_info = (cheese, self.field);
            score += 1;
        }
    }
}
