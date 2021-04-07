use std::collections::VecDeque;

use crate::maze::{Coord, Field};
use crate::square::Square;

pub fn find_paths(
    mut maze: Vec<Vec<Square>>,
    start_info: (Coord, Field),
    target: Square,
) -> String {
    let (coord, field) = start_info;
    let mut pos_queue = VecDeque::<Coord>::new();
    let mut path_queue = VecDeque::<String>::new();
    pos_queue.push_back(coord);
    path_queue.push_back("S".to_string());
    while let Some(pos) = pos_queue.pop_front() {
        let coord = pos;
        let path = path_queue.pop_front().unwrap();
        if coord.0 > 0 {
            let up = coord.0 - 1;
            let path = update_queues(
                &mut maze,
                &mut pos_queue,
                &mut path_queue,
                path.clone(),
                'U',
                Coord(up, coord.1),
                target,
            );
            if let Some(p) = path {
                return p;
            }
        }
        if coord.0 < field.height - 1 {
            let down = coord.0 + 1;
            let path = update_queues(
                &mut maze,
                &mut pos_queue,
                &mut path_queue,
                path.clone(),
                'D',
                Coord(down, coord.1),
                target,
            );
            if let Some(p) = path {
                return p;
            }
        }
        if coord.1 > 0 {
            let left = coord.1 - 1;
            let path = update_queues(
                &mut maze,
                &mut pos_queue,
                &mut path_queue,
                path.clone(),
                'L',
                Coord(coord.0, left),
                target,
            );
            if let Some(p) = path {
                return p;
            }
        }
        if coord.1 < field.width - 1 {
            let right = coord.1 + 1;
            let path = update_queues(
                &mut maze,
                &mut pos_queue,
                &mut path_queue,
                path.clone(),
                'R',
                Coord(coord.0, right),
                target,
            );
            if let Some(p) = path {
                return p;
            }
        }
    }
    String::new()
}

fn update_queues(
    maze: &mut Vec<Vec<Square>>,
    pos_queue: &mut VecDeque<Coord>,
    path_queue: &mut VecDeque<String>,
    path: String,
    direction: char,
    coord: Coord,
    target: Square,
) -> Option<String> {
    let p = maze[coord.0][coord.1];
    if p == target {
        return Some(format!("{}{}", path, direction));
    } else if p == Square::Empty {
        maze[coord.0][coord.1] = Square::Visited;
        pos_queue.push_back(coord);
        path_queue.push_back(format!("{}{}", path, direction));
    }
    None
}
