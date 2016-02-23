use std::fmt;
use std::fmt::Display;
use std::collections::HashSet;
use core::hash::Hash;
use core::cmp::Eq;

extern "rust-intrinsic" {
    fn ctlz<T>(x: T) -> T;
    fn cttz<T>(x: T) -> T;
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

fn vec_to_set<T: Eq + Hash>(vec: &mut Vec<T>) -> HashSet<T> {
    let mut set = HashSet::new();
    for m in vec.drain(..) { set.insert(m); }
    set
}

fn set_to_vec<T: Eq + Hash>(set: &mut HashSet<T>) -> Vec<T> {
    let mut vec = Vec::new();
    for m in set.drain() { vec.push(m); }
    vec
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum Player {
    Red,
    Blue
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "player {:?}", self)
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
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
    rocks: Vec<u64>,

    blues_r: Vec<u64>,
    reds_r: Vec<u64>,
    rocks_r: Vec<u64>,
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut grid = String::new();
        grid.push_str("\n");
        for (row, ((&blue, &red), &rock)) in self.blues.iter()
                                     .zip(self.reds.iter())
                                     .zip(self.rocks.iter())
                                     .enumerate() {
            let mut col = 0;
            while col < self.size {
                let mut val = '-';
                let mask = 1 << col;
                if (blue & mask > 0) {
                    val = 'b';
                } else if (red & mask > 0) {
                    val = 'r';
                } else if (rock & mask > 0) {
                    val = '#';
                }
                grid.push_str(&*format!("{} ", &val));
                col += 1;
            }
            grid.push_str("\n");
        }
        write!(f, "{}Turn: {}", grid, self.turn)
    }
}

impl Board {
    pub fn new(size: usize) -> Board {
        Board {
            turn: Player::Blue,
            size: size,
            blues: vec![0; size],
            reds: vec![0; size],
            rocks: vec![0; size],
            blues_r: vec![0; size],
            reds_r: vec![0; size],
            rocks_r: vec![0; size],
        }
    }

    // Get horizontal moves, given the board masks
    fn get_axis_moves(&self, reds: &Vec<u64>, blues: &Vec<u64>,
                      rocks: &Vec<u64>, transpose: bool) -> Vec<Move> {
        let mut moves = Vec::new();
        for (row, ((&x, &y), &z)) in blues.iter()
                                          .zip(reds.iter())
                                          .zip(rocks.iter())
                                          .enumerate() {
            let combined = x | y | z;
            if let Some(zeros) = leading_zeros(combined) {
                let col = 63 - zeros;
                if col < self.size {
                    moves.push(Move {
                        row: if transpose { col } else { row },
                        col: if transpose { row } else { col },
                        player: self.turn.clone()
                    });
                }
            }
            if let Some(zeros) = trailing_zeros(combined) {
                let col = if zeros > self.size { Some(self.size - 1) }
                          else if zeros < self.size { Some(zeros) }
                          else { None };
                if let Some(c) = col {
                    moves.push(Move {
                        row: if transpose { c } else { row },
                        col: if transpose { row } else { c },
                        player: self.turn.clone()
                    });
                }
            }
        }
        moves
    }

    // Get all moves accessible from horizontal and vertical axes
    pub fn get_moves(&self) -> Vec<Move> {
        let mut moves = self.get_axis_moves(&self.blues, &self.reds, &self.rocks, false);
        let mut ortho_moves = self.get_axis_moves(&self.blues_r, &self.reds_r, &self.rocks_r, true);
        moves.append(&mut ortho_moves);
        set_to_vec(&mut vec_to_set(&mut moves))
    }

    pub fn set_move(&mut self, m: Move) {
        self.set(m.row, m.col, Some(m.player));
    }

    pub fn set(&mut self, row: usize, col: usize, val: Option<Player>) {
        if let Some(color) = val {
            match color {
                Player::Blue => {
                    self.blues[row] |= 1 << col;
                    self.blues_r[col] |= 1 << row;
                    self.reds[row] &= !0 & (0 << col);
                    self.reds_r[col] &= !0 & (0 << row);
                },
                Player::Red => {
                    self.reds[row] |= 1 << col;
                    self.reds_r[col] |= 1 << row;
                    self.blues[row] &= !0 & (0 << col);
                    self.blues_r[col] &= !0 & (0 << row);
                }
            }
        } else {
            self.blues[row] &= !0 & (0 << col);
            self.blues_r[col] &= !0 & (0 << row);
            self.reds[row] &= !0 & (0 << col);
            self.reds_r[col] &= !0 & (0 << row);
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<Player> {
        if self.blues[row] & (1 << col) != 0 { return Some(Player::Blue) };
        if self.reds[row] & (1 << col) != 0 { return Some(Player::Red) };
        None
    }

    // Returns whether or not current Board state is a win for `player`
    pub fn eval(&self, player: Player) -> bool {
        // TODO
        true
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
    assert_eq!(Some(63), leading_zeros(0));
    assert_eq!(Some(62), leading_zeros(1));
    assert_eq!(None, leading_zeros(!0));
    assert_eq!(None, leading_zeros(0x8000000000000000));
    assert_eq!(Some(0), leading_zeros(0x6000000000000000));
}

#[test]
fn test_trailing_zeros() {
    assert_eq!(Some(63), trailing_zeros(0));
    assert_eq!(None, trailing_zeros(1));
    assert_eq!(None, trailing_zeros(!0));
    assert_eq!(Some(0), trailing_zeros(2));
}

#[test]
fn test_get_moves_basic_2() {
    let mut b = Board::new(2);
    b.set(0, 0, Some(Player::Blue)); // B B
    b.set(0, 1, Some(Player::Blue)); // 0 0
    let mut expected = vec![
        Move { row: 1, col: 0, player: Player::Blue },
        Move { row: 1, col: 1, player: Player::Blue },
    ];
    assert_eq!(vec_to_set(&mut expected), vec_to_set(&mut b.get_moves()));
}

#[test]
fn test_get_moves_basic_3() {
    let mut b = Board::new(3);
    b.set(0, 0, Some(Player::Blue)); // B 0 0
    b.set(1, 1, Some(Player::Blue)); // 0 B 0
    b.set(2, 2, Some(Player::Blue)); // 0 0 B
    let mut expected = vec![
        Move { row: 0, col: 1, player: Player::Blue },
        Move { row: 1, col: 2, player: Player::Blue },
        Move { row: 1, col: 0, player: Player::Blue },
        Move { row: 2, col: 1, player: Player::Blue },
    ];
    assert_eq!(vec_to_set(&mut expected), vec_to_set(&mut b.get_moves()));
}

#[test]
fn test_get_moves_empty_2() {
    let mut b = Board::new(2);
    let mut expected = vec![
        Move { row: 0, col: 0, player: Player::Blue },
        Move { row: 0, col: 1, player: Player::Blue },
        Move { row: 1, col: 0, player: Player::Blue },
        Move { row: 1, col: 1, player: Player::Blue },
    ];
    assert_eq!(vec_to_set(&mut expected), vec_to_set(&mut b.get_moves()));
}

#[test]
fn test_get_moves_empty_3() {
    let mut b = Board::new(3);
    let mut expected = vec![
        Move { row: 0, col: 0, player: Player::Blue },
        Move { row: 0, col: 1, player: Player::Blue },
        Move { row: 0, col: 2, player: Player::Blue },
        Move { row: 1, col: 0, player: Player::Blue },
        Move { row: 1, col: 2, player: Player::Blue },
        Move { row: 2, col: 0, player: Player::Blue },
        Move { row: 2, col: 1, player: Player::Blue },
        Move { row: 2, col: 2, player: Player::Blue },
    ];
    assert_eq!(vec_to_set(&mut expected), vec_to_set(&mut b.get_moves()));
}
