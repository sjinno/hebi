mod maze;
use maze::{Maze, MazeBuilder};

fn main() {
    println!("\x1B[2J\x1B[1;1H");
    let mut maze1: Maze = MazeBuilder::new(32, 32).place_obstacles(150).build();
    maze1.start_game_loop();
}
