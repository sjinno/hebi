mod maze;
use maze::Maze;

fn main() {
    println!("\x1B[2J\x1B[1;1H");
    let (m, n) = (32, 32);
    let mut maze1 = Maze::new(&m, &n);
    maze1.start_game_loop();
}
