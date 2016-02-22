//use rustc_data_structures::bitvec::BitMatrix;
//pub unsafe extern "rust-intrinsic" fn ctlz<T>(x: T) -> T
use std::fmt;
use std::fmt::Display;

extern "rust-intrinsic" {
    fn ctlz<T>(x: T) -> T;
    fn cttz<T>(x: T) -> T;
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Player {
    Red,
    Blue
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "player {:?}", self)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Move {
    row: usize,
    col: usize,
    player: Player
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.row, self.col, self.player)
    }
}

#[derive(Clone)]
pub struct Board {
    size: usize,
    turn: Player,
    blues: Vec<u64>,
    reds: Vec<u64>,
    rocks: Vec<u64>
}

impl Board {
    pub fn new(size: usize) -> Board {
        Board {
            turn: Player::Blue,
            size: size,
            blues: vec![0; size],
            reds: vec![0; size],
            rocks: vec![0; size]
        }
    }

    fn leading_zeros(x: u64) -> Option<usize> {
        unsafe {
            let i = ctlz(x) as usize;
            if i > 0 { Some(i - 1) } else { None }
        }
    }

    fn trailing_zeros(x: u64) -> Option<usize> {
        unsafe {
            let i = cttz(x) as usize;
            if i > 0 { Some(i - 1) } else { None }
        }
    }

    pub fn get_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        for (row, ((&x, &y), &z)) in self.blues.iter()
                                     .zip(self.reds.iter())
                                     .zip(self.rocks.iter())
                                     .enumerate() {
            let combined = x | y | z;
            println!("{:?}", combined);
            if let Some(zeros) = Self::leading_zeros(combined) {
                let col = 63 - zeros;
                if col < self.size {
                    moves.push(Move {
                        row: row,
                        col: col,
                        player: self.turn.clone()
                    });
                }
            }
            if let Some(zeros) = Self::trailing_zeros(combined) {
                let col = if zeros > self.size { Some(self.size - 1) }
                          else if zeros < self.size { Some(zeros) }
                          else { None };
                if let Some(c) = col {
                    moves.push(Move {
                        row: row,
                        col: c,
                        player: self.turn.clone()
                    });
                }
            }
        }
        moves
    }

    pub fn set_move(&mut self, m: Move) {
        self.set(m.row, m.col, Some(m.player));
    }

    pub fn set(&mut self, row: usize, col: usize, val: Option<Player>) {
        if let Some(color) = val {
            match color {
                Player::Blue => {
                    self.blues[row] |= 1 << col;
                    self.reds[row] &= !0 & (0 << col);
                },
                Player::Red => {
                    self.reds[row] |= 1 << col;
                    self.blues[row] &= !0 & (0 << col);
                }
            }
        } else {
            self.blues[row] &= !0 & (0 << col);
            self.reds[row] &= !0 & (0 << col);
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<Player> {
        if self.blues[row] & (1 << col) != 0 { return Some(Player::Blue) };
        if self.reds[row] & (1 << col) != 0 { return Some(Player::Red) };
        None
    }
}

#[test]
fn test_get_set() {
    let mut b = Board::new(4);

    assert_eq!(None, b.get(1, 0));

    b.set(0, 0, Some(Player::Blue));
    assert_eq!(Some(Player::Blue), b.get(0, 0));

    assert_eq!(None, b.get(1, 0));

    b.set(0, 0, Some(Player::Red));
    assert_eq!(Some(Player::Red), b.get(0, 0));

    b.set(0, 0, None);
    assert_eq!(None, b.get(0, 0));
}

#[test]
fn test_clone() {
    let mut b = Board::new(4);

    assert_eq!(None, b.get(0, 0));

    b.set(0, 0, Some(Player::Blue));
    assert_eq!(Some(Player::Blue), b.get(0, 0));

    let mut c = b.clone();
    assert_eq!(Some(Player::Blue), c.get(0, 0));

    // Change c and makes sure b is unchanged
    c.set(0, 0, Some(Player::Red));
    assert_eq!(Some(Player::Blue), b.get(0, 0));
}

#[test]
fn test_leading_zeros() {
    assert_eq!(Some(63), Board::leading_zeros(0));
    assert_eq!(Some(62), Board::leading_zeros(1));
    assert_eq!(None, Board::leading_zeros(!0));
    assert_eq!(None, Board::leading_zeros(0x8000000000000000));
    assert_eq!(Some(0), Board::leading_zeros(0x6000000000000000));
}

#[test]
fn test_trailing_zeros() {
    assert_eq!(Some(63), Board::trailing_zeros(0));
    assert_eq!(None, Board::trailing_zeros(1));
    assert_eq!(None, Board::trailing_zeros(!0));
    assert_eq!(Some(0), Board::trailing_zeros(2));
}

#[test]
fn test_get_moves_basic_2() {
    let mut b = Board::new(2);
    b.set(0, 0, Some(Player::Blue)); // B B
    b.set(0, 1, Some(Player::Blue)); // 0 0
    let expected = vec![
        Move { row: 1, col: 0, player: Player::Blue },
        Move { row: 1, col: 1, player: Player::Blue },
    ];
    assert_eq!(expected, b.get_moves());
}

#[test]
fn test_get_moves_basic_3() {
    let mut b = Board::new(3);
    b.set(0, 0, Some(Player::Blue)); // B 0 0
    b.set(1, 1, Some(Player::Blue)); // 0 B 0
    b.set(2, 2, Some(Player::Blue)); // 0 0 B
    let expected = vec![
        Move { row: 0, col: 1, player: Player::Blue },
        Move { row: 0, col: 2, player: Player::Blue },
        Move { row: 1, col: 0, player: Player::Blue },
        Move { row: 1, col: 2, player: Player::Blue },
        Move { row: 2, col: 0, player: Player::Blue },
        Move { row: 2, col: 1, player: Player::Blue },
    ];
    assert_eq!(expected, b.get_moves());
}

#[test]
fn test_get_moves_empty() {
    let mut b = Board::new(2);
    let expected = vec![
        Move { row: 0, col: 0, player: Player::Blue },
        Move { row: 0, col: 1, player: Player::Blue },
        Move { row: 1, col: 0, player: Player::Blue },
        Move { row: 1, col: 1, player: Player::Blue },
    ];
    assert_eq!(expected, b.get_moves());
}
