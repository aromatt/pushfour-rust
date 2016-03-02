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

impl Display for Player {
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
        write!(f, "Move ({}, {}, {})", self.row, self.col, self.player)
    }
}

#[derive(Clone, Debug)]
pub struct Coord(usize, usize);

fn rotate_cw(size: usize, row: usize, col: usize) -> (usize, usize) {
    println!("rotate {} {} {}", size, row, col);
    (col, size - row - 1)
}

#[derive(Clone)]
pub struct Board {
    size: usize,
    turn: Player,

    blues: Vec<u64>,
    reds: Vec<u64>,
    rocks: Vec<u64>,

    blues_invert: Vec<u64>,
    reds_invert: Vec<u64>,
    rocks_invert: Vec<u64>,

    pub diag_lookup: Vec<Vec<Coord>>,
    blues_diag: Vec<u64>,
    reds_diag: Vec<u64>,
    rocks_diag: Vec<u64>,

    pub diag_lookup_rot: Vec<Vec<Coord>>,
    blues_diag_rot: Vec<u64>,
    reds_diag_rot: Vec<u64>,
    rocks_diag_rot: Vec<u64>,
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
        let diag_rows = size * 2 - 1;
        let mut b = Board {
            turn: Player::Blue,
            size: size,

            blues: vec![0; size],
            reds: vec![0; size],
            rocks: vec![0; size],

            blues_invert: vec![0; size],
            reds_invert: vec![0; size],
            rocks_invert: vec![0; size],

            diag_lookup: vec![vec![Coord(0, 0); size]; size],
            blues_diag: vec![0; diag_rows],
            reds_diag: vec![0; diag_rows],
            rocks_diag: vec![0; diag_rows],

            diag_lookup_rot: vec![vec![Coord(0, 0); size]; size],
            blues_diag_rot: vec![0; diag_rows],
            reds_diag_rot: vec![0; diag_rows],
            rocks_diag_rot: vec![0; diag_rows],
        };
        b.init_diag_lookups();
        b
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
        println!("init diag lookups");
        let mut key_row_reset = 1;
        let mut key_col_reset = 1;
        let mut key_row = 0;
        let mut key_col = 0;
        let mut val_row = 0;
        let mut val_col = 0;
        let mut total = 0;
        while total < self.size * self.size {
            println!("total {}", total);
            println!("keys {} {} | {} {}", key_row, key_col, key_row_reset, key_col_reset);
            println!("vals {} {}", val_row, val_col);

            self.diag_lookup[key_row][key_col] = Coord(val_row, val_col);
            let (key_row_rot, key_col_rot) = rotate_cw(self.size, key_row, key_col);
            self.diag_lookup_rot[key_row_rot][key_col_rot] = Coord(val_row, val_col);

            // Reset from top row to the left column
            if key_row == 0 && key_row_reset < self.size {
                println!("Reset from top row to the left column, row {}", key_row_reset);
                key_row = key_row_reset;
                key_col = 0;
                key_row_reset += 1;
                val_col = 0;
                val_row += 1;
            // Reset from the right column to the bottom row
            } else if key_col == self.size - 1 && key_col_reset < self.size {
                println!("Reset from right col to bottom row, col {}", key_col_reset);
                key_col = key_col_reset;
                key_row = self.size - 1;
                key_col_reset += 1;
                val_col = 0;
                val_row += 1;
            // Normal traversal up and to the right
            } else {
                println!("normal traversal");
                key_row -= 1;
                key_col += 1;
                val_col += 1;
            }
            total += 1;
        }
        println!("done with init diag lookups");
    }

    // Get horizontal moves, given the board masks.
    // (call with both horizontal and vertical representations to get all moves)
    fn get_axis_moves(&self, reds: &Vec<u64>, blues: &Vec<u64>,
                      rocks: &Vec<u64>, transpose: bool) -> Vec<Move> {
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

    // Get all moves accessible from horizontal and vertical axes
    pub fn get_moves(&self) -> Vec<Move> {
        let mut moves = self.get_axis_moves(&self.blues, &self.reds, &self.rocks, false);
        let mut ortho_moves = self.get_axis_moves(&self.blues_invert, &self.reds_invert, &self.rocks_invert, true);
        moves.append(&mut ortho_moves);
        set_to_vec(&mut vec_to_set(&mut moves))
    }

    #[allow(dead_code)]
    pub fn set_move(&mut self, m: Move) {
        self.set(m.row, m.col, Some(m.player));
    }

    // TODO create new struct for piece type; use that here.
    #[allow(dead_code)]
    pub fn set(&mut self, row: usize, col: usize, val: Option<Player>) {
        let (row_invert, col_invert) = (col, row);
        let Coord(drow, dcol) = self.diag_lookup[row][col];
        let Coord(drow_rot, dcol_rot) = self.diag_lookup_rot[row][col];
        if let Some(color) = val {
            match color {
                Player::Blue => {
                    // Set blues
                    self.blues[row] |= 1 << col;
                    self.blues_invert[row_invert] |= 1 << col_invert;
                    self.blues_diag[drow] |= 1 << dcol;
                    self.blues_diag_rot[drow_rot] |= 1 << dcol_rot;

                    // Clear reds
                    self.reds[row] &= !0 & (0 << col);
                    self.reds_invert[row_invert] &= !0 & (0 << col_invert);
                    self.reds_diag[drow] &= !0 & (0 << dcol);
                    self.reds_diag_rot[drow_rot] &= !0 & (0 << dcol_rot);
                    // TODO rocks
                },
                Player::Red => {
                    // Set reds
                    self.reds[row] |= 1 << col;
                    self.reds_invert[row_invert] |= 1 << col_invert;
                    self.reds_diag[drow] |= 1 << dcol;
                    self.reds_diag_rot[drow_rot] |= 1 << dcol_rot;

                    // Clear blues
                    self.blues[row] &= !0 & (0 << col);
                    self.blues_invert[row_invert] &= !0 & (0 << col_invert);
                    self.blues_diag[drow] &= !0 & (0 << dcol);
                    self.blues_diag_rot[drow_rot] &= !0 & (0 << dcol_rot);
                    // TODO rocks
                }
            }
        } else {
            // Clear all TODO rocks
            self.reds[row] &= !0 & (0 << col);
            self.reds_invert[row_invert] &= !0 & (0 << col_invert);
            self.blues[row] &= !0 & (0 << col);
            self.blues_invert[row_invert] &= !0 & (0 << col_invert);
            self.reds_diag[drow] &= !0 & (0 << dcol);
            self.reds_diag_rot[drow_rot] &= !0 & (0 << dcol_rot);
            self.blues_diag[drow] &= !0 & (0 << dcol);
            self.blues_diag_rot[drow_rot] &= !0 & (0 << dcol_rot);
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<Player> {
        if self.blues[row] & (1 << col) != 0 { return Some(Player::Blue) };
        if self.reds[row] & (1 << col) != 0 { return Some(Player::Red) };
        None
    }

    // Returns whether or not current Board state is a win for `player`
    #[allow(unused_variables)]
    #[allow(dead_code)]
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
    println!("{:?}", b);
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
