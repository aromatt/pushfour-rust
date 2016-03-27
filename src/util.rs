use std::fmt;
use std::fmt::Display;

pub const BOARD_SIZE: usize = 8;

pub const BOARD_DIAG_SIZE: usize = BOARD_SIZE * 2 - 1;
pub const BLUE_CHAR: char = 'b';
pub const RED_CHAR: char = 'r';
pub const ROCK_CHAR: char = '#';


#[derive(Clone, Copy, Debug)]
pub struct Coord(pub usize, pub usize);

pub fn rotate_cw(size: usize, row: usize, col: usize) -> (usize, usize) {
    (col, size - row - 1)
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum Piece {
    Red,
    Blue,
    Rock
}

impl Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Piece {:?}", self)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Player {
    Red,
    Blue
}

impl Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Player {:?}", self)
    }
}

impl Player {
    pub fn to_piece(&self) -> Piece {
        match *self {
            Player::Red => Piece::Red,
            Player::Blue => Piece::Blue,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct Move {
    pub row: usize,
    pub col: usize,
    pub player: Player
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Move ({}, {}, {})", self.row, self.col, self.player)
    }
}
