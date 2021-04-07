use std::fmt::{self, Write};

use termion::color::{self, LightGreen, Reset, Yellow}; // termion = "1.5.6"

use crate::color;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Square {
    Empty,
    Visited,
    Block,
    Snake,
    Bait,
}

impl fmt::Display for Square {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Square::Bait => write!(fmt, "{}", color!(bait)),
            Square::Snake => write!(fmt, "{}", color!(snake)),
            Square::Block => fmt.write_char('■'),
            Square::Empty | Square::Visited => fmt.write_char(' '),
        }
    }
}
