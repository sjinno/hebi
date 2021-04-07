use std::collections::VecDeque;

use crate::maze::{Coord, Field, Square};

pub fn find_paths(
    mut maze: Vec<Vec<Square>>,
    start_info: (Coord, Field),
    target: Square,
) -> String {
    let (coord, field) = start_info;
    let mut pos_queue = VecDeque::<(usize, usize)>::new();
    let mut path_queue = VecDeque::<String>::new();
    pos_queue.push_back((coord.0, coord.1));
    path_queue.push_back("S".to_string());
    while let Some(pos) = pos_queue.pop_front() {
        let (r, c) = pos;
        let path = path_queue.pop_front().unwrap();
        let (row, col) = (r, c);
        if row > 0 {
            let up = row - 1;
            let path = update_queues(
                &mut maze,
                &mut pos_queue,
                &mut path_queue,
                path.clone(),
                'U',
                up,
                col,
                target.clone(),
            );
            if let Some(p) = path {
                return p;
            }
        }
        if row < field.height - 1 {
            let down = row + 1;
            let path = update_queues(
                &mut maze,
                &mut pos_queue,
                &mut path_queue,
                path.clone(),
                'D',
                down,
                col,
                target.clone(),
            );
            if let Some(p) = path {
                return p;
            }
        }
        if col > 0 {
            let left = col - 1;
            let path = update_queues(
                &mut maze,
                &mut pos_queue,
                &mut path_queue,
                path.clone(),
                'L',
                row,
                left,
                target.clone(),
            );
            if let Some(p) = path {
                return p;
            }
        }
        if col < field.width - 1 {
            let right = col + 1;
            let path = update_queues(
                &mut maze,
                &mut pos_queue,
                &mut path_queue,
                path.clone(),
                'R',
                row,
                right,
                target.clone(),
            );
            if let Some(p) = path {
                return p;
            }
        }
    }
    String::new()
}

pub fn update_queues(
    maze: &mut Vec<Vec<Square>>,
    pos_queue: &mut VecDeque<(usize, usize)>,
    path_queue: &mut VecDeque<String>,
    path: String,
    direction: char,
    row: usize,
    col: usize,
    target: Square,
) -> Option<String> {
    let p = maze[row][col];
    if p == target {
        return Some(format!("{}{}", path, direction));
    } else if p == Square::Empty {
        maze[row][col] = Square::Visited;
        pos_queue.push_back((row, col));
        path_queue.push_back(format!("{}{}", path, direction));
    }
    None
}
