use std::collections::VecDeque;
use std::thread;
use std::time::Duration;

use crate::clear;
use crate::maze::{Maze, Square};
use crate::path_finder::*;

pub trait Snake {
    fn activate_snake(&mut self);
}

impl Snake for Maze {
    fn activate_snake(&mut self) {
        let snake = self.place_object(Square::Snake);
        let mut start_info = (snake.0, snake.1, self.m, self.n);
        let mut snake_size = VecDeque::<(usize, usize)>::new();
        let mut score = 0;
        loop {
            let cheese = self.place_object(Square::Bait);
            let path = find_paths(self.maze.clone(), start_info, Square::Bait); // Only get the shortest path.
            draw(path, self.maze.clone(), start_info, &score, &mut snake_size);
            thread::sleep(Duration::from_nanos(1));
            self.maze[cheese.0][cheese.1] = Square::Empty;
            self.maze[snake.0][snake.1] = Square::Empty;
            start_info = (cheese.0, cheese.1, self.m, self.n);
            score += 1;
        }
    }
}

fn draw(
    path: String,
    maze: Vec<Vec<Square>>,
    start_info: (usize, usize, usize, usize),
    score: &usize,
    snake_size: &mut VecDeque<(usize, usize)>,
) {
    let mut mz = maze;
    let (mut r, mut c, _, _) = start_info;

    for p in path.chars() {
        if snake_size.len() < 6 {
            snake_size.push_back((r, c));
            let (tail_r, tail_c) = snake_size.pop_front().unwrap();
            mz[tail_r][tail_c] = Square::Empty;
        } else {
            let (tail_r, tail_c) = snake_size.pop_front().unwrap();
            mz[tail_r][tail_c] = Square::Empty;
        }

        match p {
            'U' => r -= 1,
            'D' => r += 1,
            'L' => c -= 1,
            'R' => c += 1,
            _ => {}
        }
        mz[r][c] = Square::Snake;

        println!("Current score: {}", score);
        for row in &mz {
            for col in row {
                print!("{} ", col);
            }
            println!();
        }

        thread::sleep(Duration::from_millis(60));
        clear!(all);
        snake_size.push_back((r, c));
    }
}
