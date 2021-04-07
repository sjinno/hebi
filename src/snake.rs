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
        let snake_coord = self.place_object(Square::Snake);
        let mut start_info = (snake_coord, self.field);
        let mut snake_size = VecDeque::<(usize, usize)>::new();
        let mut score = 0;
        loop {
            let bait_coord = self.place_object(Square::Bait);
            let path = find_paths(self.clone(), start_info, Square::Bait); // Only get the shortest path.
            Self::draw(&path, &mut self.clone(), start_info, score, &mut snake_size);
            thread::sleep(Duration::from_nanos(1));
            self.cells[bait_coord.0][bait_coord.1] = Square::Empty;
            self.cells[snake_coord.0][snake_coord.1] = Square::Empty;
            start_info = (bait_coord, self.field);
            score += 1;
        }
    }
}
