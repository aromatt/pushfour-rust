extern crate rustc_data_structures;
extern crate core;

use std::fmt;
use std::fmt::Display;
use std::collections::HashSet;
use self::core::hash::Hash;
use self::core::cmp::Eq;

const BOARD_SIZE: usize = 6;
const BOARD_DIAG_SIZE: usize = BOARD_SIZE * 2 - 1;

fn add(a: i32, b: i32) -> i32 {
    let c: i32;
    unsafe {
        asm!("add $2, $0"
             : "=r"(c)
             : "0"(a), "r"(b)
            );
    }
    c
}

//http://llvm.org/docs/LangRef.html#inline-asm-modifiers
//https://doc.rust-lang.org/book/inline-assembly.html
fn bsf(a: u32) -> u64 {
    let i: u64;
    unsafe {
        asm!("bsf $0"
             : "=r"(i)
             : "r"(a)
             :
             : "intel"
            );
    }
    i
}

extern "rust-intrinsic" {
    #[no_mangle]
    fn ctlz<T>(x: T) -> T;

    #[no_mangle]
    fn cttz<T>(x: T) -> T;
}

#[no_mangle]
fn leading_zeros(x: u64) -> Option<usize> {
    unsafe {
        let i = ctlz(x) as usize;
        if i > 0 { Some(i - 1) } else { None }
    }
}

#[no_mangle]
fn trailing_zeros(x: u64) -> Option<usize> {
    unsafe {
        let i = cttz(x) as usize;
        if i > 0 { Some(i - 1) } else { None }
    }
}

#[inline(always)]
fn is_row_win(mut row: u64) -> bool {
    if row == 0 { return false; }
    let mut i = 0;
    while i < 3 {
        row = row & (row >> 1);
        if row == 0 { return false; }
        i += 1;
    }
    true
}

#[inline(always)]
fn vec_to_set<T: Eq + Hash>(vec: &mut Vec<T>) -> HashSet<T> {
    let mut set = HashSet::new();
    for m in vec.drain(..) { set.insert(m); }
    set
}

#[inline(always)]
fn set_to_vec<T: Eq + Hash>(set: &mut HashSet<T>) -> Vec<T> {
    let mut vec = Vec::new();
    for m in set.drain() { vec.push(m); }
    vec
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
    fn to_piece(&self) -> Piece {
        match *self {
            Player::Red => Piece::Red,
            Player::Blue => Piece::Blue,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
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

#[derive(Clone, Copy, Debug)]
pub struct Coord(usize, usize);

fn rotate_cw(size: usize, row: usize, col: usize) -> (usize, usize) {
    (col, size - row - 1)
}

#[derive(Clone, Copy)]
pub struct Board {
    size: usize,
    turn: Player,

    blues: [u64; BOARD_SIZE],
    reds: [u64; BOARD_SIZE],
    rocks: [u64; BOARD_SIZE],

    blues_invert: [u64; BOARD_SIZE],
    reds_invert: [u64; BOARD_SIZE],
    rocks_invert: [u64; BOARD_SIZE],

    pub diag_lookup: [[Coord; BOARD_SIZE]; BOARD_SIZE],
    blues_diag: [u64; BOARD_DIAG_SIZE],
    reds_diag: [u64; BOARD_DIAG_SIZE],
    rocks_diag: [u64; BOARD_DIAG_SIZE],

    pub diag_lookup_rot: [[Coord; BOARD_SIZE]; BOARD_SIZE],
    blues_diag_rot: [u64; BOARD_DIAG_SIZE],
    reds_diag_rot: [u64; BOARD_DIAG_SIZE],
    rocks_diag_rot: [u64; BOARD_DIAG_SIZE],
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut grid = String::new();
        grid.push_str("\n");
        let mut row = 0;
        while row < self.size {
            let mut col = 0;
            let blue_row = self.blues[row];
            let red_row = self.reds[row];
            let rock_row = self.rocks[row];
            while col < self.size {
                let mut val = '-';
                let mask = 1 << col;
                if blue_row & mask > 0 {
                    val = 'b';
                } else if red_row & mask > 0 {
                    val = 'r';
                } else if rock_row & mask > 0 {
                    val = '#';
                }
                grid.push_str(&*format!("{} ", &val));
                col += 1;
            }
            grid.push_str("\n");
            row += 1;
        }
        write!(f, "{}Turn: {}", grid, self.turn)
    }
}

impl Board {
    pub fn new(size: usize) -> Board {
        let mut b = Board {
            turn: Player::Blue,
            size: size,

            blues: [0; BOARD_SIZE],
            reds: [0; BOARD_SIZE],
            rocks: [0; BOARD_SIZE],

            blues_invert: [0; BOARD_SIZE],
            reds_invert: [0; BOARD_SIZE],
            rocks_invert: [0; BOARD_SIZE],

            diag_lookup: [[Coord(0, 0); BOARD_SIZE]; BOARD_SIZE],
            blues_diag: [0; BOARD_DIAG_SIZE],
            reds_diag: [0; BOARD_DIAG_SIZE],
            rocks_diag: [0; BOARD_DIAG_SIZE],

            diag_lookup_rot: [[Coord(0, 0); BOARD_SIZE]; BOARD_SIZE],
            blues_diag_rot: [0; BOARD_DIAG_SIZE],
            reds_diag_rot: [0; BOARD_DIAG_SIZE],
            rocks_diag_rot: [0; BOARD_DIAG_SIZE],
        };
        b.init_diag_lookups();
        b
    }

    pub fn next_turn(&mut self) {
        if self.turn == Player::Blue {
            self.turn = Player::Red;
        } else {
            self.turn = Player::Blue;
        }
    }

    /* Need two diagonal representations of the board, and a lookup tables.
     * The lookup tables are used for setting bits in the representations.
     * The representations are only used for detecting diagonal win states.
     *
     *       00            02
     *     10  01        01  12
     *   20  11  02    00  11  22   -- Representations
     *     21  12        10  21
     *       22            22
     *
     *    00 11 22      20 10 00
     *    10 21 31      30 21 11    -- Lookup tables
     *    20 30 40      40 31 22
     *
     *      (1)           (2)
     *
     *    Alternative lookup table instead of (2), which results in an inversion
     *    of representation (1). This lookup table is obtained by flipping (1)'s
     *    lookup table about its middle row:
     *
     *      20 30 40
     *      10 21 31
     *      00 11 22
     */
    fn init_diag_lookups(&mut self) {
        //println!("init diag lookups");
        let mut key_row_reset = 1;
        let mut key_col_reset = 1;
        let mut key_row = 0;
        let mut key_col = 0;
        let mut val_row = 0;
        let mut val_col = 0;
        let mut total = 0;
        while total < self.size * self.size {

            self.diag_lookup[key_row][key_col] = Coord(val_row, val_col);
            let (key_row_rot, key_col_rot) = rotate_cw(self.size, key_row, key_col);
            self.diag_lookup_rot[key_row_rot][key_col_rot] = Coord(val_row, val_col);

            // Reset from top row to the left column
            if key_row == 0 && key_row_reset < self.size {
                key_row = key_row_reset;
                key_col = 0;
                key_row_reset += 1;
                val_col = 0;
                val_row += 1;
            // Reset from the right column to the bottom row
            } else if key_col == self.size - 1 && key_col_reset < self.size {
                key_col = key_col_reset;
                key_row = self.size - 1;
                key_col_reset += 1;
                val_col = 0;
                val_row += 1;
            // Normal traversal up and to the right
            } else {
                key_row -= 1;
                key_col += 1;
                val_col += 1;
            }
            total += 1;
        }
    }

    // Get horizontal moves, given the board masks.
    // (call with both horizontal and vertical representations to get all moves)
    fn get_axis_moves(&self, reds: &[u64], blues: &[u64],
                      rocks: &[u64], transpose: bool) -> Vec<Move> {
        let mut moves = Vec::new();
        let mut row = 0;
        while row < self.size {
            let combined = blues[row] | reds[row] | rocks[row];
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
            row += 1;
        }
        moves
    }

    // Get all moves, allowing duplicates
    pub fn get_moves_dirty(&self) -> Vec<Move> {
        let mut moves = self.get_axis_moves(&self.blues, &self.reds, &self.rocks, false);
        let mut ortho_moves = self.get_axis_moves(&self.blues_invert, &self.reds_invert,
                                                  &self.rocks_invert, true);
        moves.append(&mut ortho_moves);
        moves
    }

    // Get all moves, in a HashSet
    pub fn get_moves_set(&self) -> HashSet<Move> {
        vec_to_set(&mut self.get_moves_dirty())
    }

    // Get all moves, as a uniq'd Veq
    pub fn get_moves(&self) -> Vec<Move> {
        set_to_vec(&mut self.get_moves_set())
    }

    pub fn set_move(&mut self, m: Move) {
        self.set(m.row, m.col, Some(m.player.to_piece()));
    }

    pub fn set(&mut self, row: usize, col: usize, val: Option<Piece>) {
        let (row_invert, col_invert) = (col, row);
        let Coord(drow, dcol) = self.diag_lookup[row][col];
        let Coord(drow_rot, dcol_rot) = self.diag_lookup_rot[row][col];
        if let Some(color) = val {
            match color {
                Piece::Blue => {
                    // Set blues
                    self.blues[row] |= 1 << col;
                    self.blues_invert[row_invert] |= 1 << col_invert;
                    self.blues_diag[drow] |= 1 << dcol;
                    self.blues_diag_rot[drow_rot] |= 1 << dcol_rot;

                    // Clear reds
                    self.reds[row] &= !0 ^ (1 << col);
                    self.reds_invert[row_invert] &= !0 ^ (1 << col_invert);
                    self.reds_diag[drow] &= !0 ^ (1 << dcol);
                    self.reds_diag_rot[drow_rot] &= !0 ^ (1 << dcol_rot);

                    // Clear rocks
                    self.rocks[row] &= !0 ^ (1 << col);
                    self.rocks_invert[row_invert] &= !0 ^ (1 << col_invert);
                },
                Piece::Red => {
                    // Set reds
                    self.reds[row] |= 1 << col;
                    self.reds_invert[row_invert] |= 1 << col_invert;
                    self.reds_diag[drow] |= 1 << dcol;
                    self.reds_diag_rot[drow_rot] |= 1 << dcol_rot;

                    // Clear blues
                    self.blues[row] &= !0 ^ (1 << col);
                    self.blues_invert[row_invert] &= !0 ^ (1 << col_invert);
                    self.blues_diag[drow] &= !0 ^ (1 << dcol);
                    self.blues_diag_rot[drow_rot] &= !0 ^ (1 << dcol_rot);

                    // Clear rocks
                    self.rocks[row] &= !0 ^ (1 << col);
                    self.rocks_invert[row_invert] &= !0 ^ (1 << col_invert);
                },
                Piece::Rock => {
                    // Set rocks
                    self.rocks[row] |= 1 << col;
                    self.rocks_invert[row_invert] |= 1 << col_invert;

                    // Clear blues
                    self.blues[row] &= !0 ^ (1 << col);
                    self.blues_invert[row_invert] &= !0 ^ (1 << col_invert);
                    self.blues_diag[drow] &= !0 ^ (1 << dcol);
                    self.blues_diag_rot[drow_rot] &= !0 ^ (1 << dcol_rot);

                    // Clear reds
                    self.reds[row] &= !0 ^ (1 << col);
                    self.reds_invert[row_invert] &= !0 ^ (1 << col_invert);
                    self.reds_diag[drow] &= !0 ^ (1 << dcol);
                    self.reds_diag_rot[drow_rot] &= !0 ^ (1 << dcol_rot);
                }
            }
        } else {
            self.reds[row] &= !0 ^ (1 << col);
            self.reds_invert[row_invert] &= !0 ^ (1 << col_invert);
            self.blues[row] &= !0 ^ (1 << col);
            self.blues_invert[row_invert] &= !0 ^ (1 << col_invert);
            self.reds_diag[drow] &= !0 ^ (1 << dcol);
            self.reds_diag_rot[drow_rot] &= !0 ^ (1 << dcol_rot);
            self.blues_diag[drow] &= !0 ^ (1 << dcol);
            self.blues_diag_rot[drow_rot] &= !0 ^ (1 << dcol_rot);
            self.rocks[row] &= !0 ^ (1 << col);
            self.rocks_invert[row_invert] &= !1 ^ (1 << col_invert);
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, row: usize, col: usize) -> Option<Piece> {
        if self.blues[row] & (1 << col) != 0 { return Some(Piece::Blue) };
        if self.reds[row] & (1 << col) != 0 { return Some(Piece::Red) };
        if self.rocks[row] & (1 << col) != 0 { return Some(Piece::Rock) };
        None
    }

    // Returns whether or not current Board state is a win for `player`
    pub fn is_win_state(&self, player: Player) -> bool {
        let (main, invert, diag, diag_rot) = match player {
            Player::Red => (&self.reds, &self.reds_invert, &self.reds_diag, &self.reds_diag_rot),
            Player::Blue => (&self.blues, &self.blues_invert, &self.blues_diag, &self.blues_diag_rot)
        };
        for row in main { if is_row_win(*row) { return true; } } // TODO print the win state in a color
        for row in invert { if is_row_win(*row) { return true; } }
        for row in diag { if is_row_win(*row) { return true; } }
        for row in diag_rot { if is_row_win(*row) { return true; } }
        false
    }
}

#[test]
fn test_get_set() {
    let mut b = Board::new(4);

    assert_eq!(None, b.get(1, 0));

    b.set(0, 0, Some(Piece::Blue));
    assert_eq!(Some(Piece::Blue), b.get(0, 0));

    assert_eq!(None, b.get(1, 0));

    b.set(0, 0, Some(Piece::Red));
    assert_eq!(Some(Piece::Red), b.get(0, 0));

    b.set(0, 0, None);
    assert_eq!(None, b.get(0, 0));

    b.set(0, 0, Some(Piece::Rock));
    assert_eq!(Some(Piece::Rock), b.get(0, 0));
}

#[test]
fn test_get_set_row() {
    let mut b = Board::new(4);
    b.set(0, 0, Some(Piece::Blue));
    assert_eq!(Some(Piece::Blue), b.get(0, 0));

    // Verify setting red in the same row doesn't clear blue
    b.set(0, 1, Some(Piece::Red));
    assert_eq!(Some(Piece::Blue), b.get(0, 0));
    assert_eq!(Some(Piece::Red), b.get(0, 1));

    // Again
    b.set(0, 2, Some(Piece::Rock));
    assert_eq!(Some(Piece::Blue), b.get(0, 0));
    assert_eq!(Some(Piece::Rock), b.get(0, 2));

}

#[test]
fn test_clone() {
    let mut b = Board::new(4);

    assert_eq!(None, b.get(0, 0));

    b.set(0, 0, Some(Piece::Blue));
    assert_eq!(Some(Piece::Blue), b.get(0, 0));

    let mut c = b.clone();
    assert_eq!(Some(Piece::Blue), c.get(0, 0));

    // Change c and makes sure b is unchanged
    c.set(0, 0, Some(Piece::Red));
    assert_eq!(Some(Piece::Blue), b.get(0, 0));
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
fn test_is_row_win() {
    assert!(!is_row_win(0));
    assert!(!is_row_win(1));
    assert!(!is_row_win(0b11));
    assert!(!is_row_win(0b111));
    assert!(is_row_win(0b1111));
    assert!(is_row_win(0b11110));
    assert!(is_row_win(0xF000000000000000));
    assert!(is_row_win(0xAA02F20011002345));
    assert!(!is_row_win(0xAA55000011002345));
    assert!(is_row_win(0x1E00000000000000));
    assert!(!is_row_win(0xE000000000000000));
}

#[test]
fn test_get_moves_basic_2() {
    let mut b = Board::new(2);
    b.set(0, 0, Some(Piece::Blue)); // B B
    b.set(0, 1, Some(Piece::Blue)); // 0 0
    let mut expected = vec![
        Move { row: 1, col: 0, player: Player::Blue },
        Move { row: 1, col: 1, player: Player::Blue },
    ];
    assert_eq!(vec_to_set(&mut expected), vec_to_set(&mut b.get_moves()));
}

#[test]
fn test_get_moves_basic_3() {
    let mut b = Board::new(3);
    b.set(0, 0, Some(Piece::Blue)); // B 0 0
    b.set(1, 1, Some(Piece::Blue)); // 0 B 0
    b.set(2, 2, Some(Piece::Blue)); // 0 0 B
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
    let b = Board::new(2);
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
    let b = Board::new(3);
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

// TODO test
//   - win states (don't forget diag)
