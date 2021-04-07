use std::collections::VecDeque;

use crate::maze::{Coord, Field, Maze};
use crate::square::Square;

pub fn find_paths(mut maze: Maze, start_info: (Coord, Field), target: Square) -> String {
    let (coord, field) = start_info;
    let mut pos_queue = VecDeque::<Coord>::new();
    let mut path_queue = VecDeque::<String>::new();
    pos_queue.push_back(coord);
    path_queue.push_back("S".to_string());
    while let Some(pos) = pos_queue.pop_front() {
        let path = path_queue.pop_front().unwrap();
        if pos.0 > 0 {
            let up = pos.0 - 1;
            let path = update_queues(
                &mut maze,
                &mut pos_queue,
                &mut path_queue,
                &path,
                'U',
                Coord(up, pos.1),
                target,
            );
            if let Some(p) = path {
                return p;
            }
        }
        if pos.0 < field.height - 1 {
            let down = pos.0 + 1;
            let path = update_queues(
                &mut maze,
                &mut pos_queue,
                &mut path_queue,
                &path,
                'D',
                Coord(down, pos.1),
                target,
            );
            if let Some(p) = path {
                return p;
            }
        }
        if pos.1 > 0 {
            let left = pos.1 - 1;
            let path = update_queues(
                &mut maze,
                &mut pos_queue,
                &mut path_queue,
                &path,
                'L',
                Coord(pos.0, left),
                target,
            );
            if let Some(p) = path {
                return p;
            }
        }
        if pos.1 < field.width - 1 {
            let right = pos.1 + 1;
            let path = update_queues(
                &mut maze,
                &mut pos_queue,
                &mut path_queue,
                &path,
                'R',
                Coord(pos.0, right),
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
    maze: &mut Maze,
    pos_queue: &mut VecDeque<Coord>,
    path_queue: &mut VecDeque<String>,
    path: &str,
    direction: char,
    coord: Coord,
    target: Square,
) -> Option<String> {
    let p = maze.maze[coord.0][coord.1];
    if p == target {
        return Some(format!("{}{}", path, direction));
    } else if p == Square::Empty {
        maze.maze[coord.0][coord.1] = Square::Visited;
        pos_queue.push_back(coord);
        path_queue.push_back(format!("{}{}", path, direction));
    }
    None
}
