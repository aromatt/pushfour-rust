extern crate rustc_data_structures;
extern crate core;

use std::fmt;
use std::fmt::Display;
use std::collections::HashSet;
use self::core::hash::Hash;
use overlay::Overlay;
use overlay::DiagLookup;

const BLUE_CHAR: char = 'b';
const RED_CHAR: char = 'r';
const ROCK_CHAR: char = '#';

/*
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
*/

//http://llvm.org/docs/LangRef.html#inline-asm-modifiers
//https://doc.rust-lang.org/book/inline-assembly.html
/*
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
*/

extern "rust-intrinsic" {
    #[allow(private_no_mangle_fns)]
    #[no_mangle]
    fn ctlz<T>(x: T) -> T;

    #[allow(private_no_mangle_fns)]
    #[no_mangle]
    fn cttz<T>(x: T) -> T;
}

#[allow(private_no_mangle_fns)]
#[no_mangle]
fn leading_zeros(x: u64) -> Option<usize> {
    unsafe {
        let i = ctlz(x) as usize;
        if i > 0 { Some(i - 1) } else { None }
    }
}

#[allow(private_no_mangle_fns)]
#[no_mangle]
fn trailing_zeros(x: u64) -> Option<usize> {
    unsafe {
        let i = cttz(x) as usize;
        if i > 0 { Some(i - 1) } else { None }
    }
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

// Representation of a pushfour board.
// It's implemented as a composition of Overlays, adding logic for getting and applying available
// moves, and some other necessities for tracking the state of the game.
#[derive(Clone)]
pub struct Board<'a> {
    size: usize,
    turn: Player,

    blues: Overlay<'a>,
    reds: Overlay<'a>,
    rocks: Overlay<'a>,
}

impl<'a> fmt::Debug for Board<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut grid = String::new();
        grid.push_str("\n+ ");
        let mut col = 0;
        while col < self.size { grid.push_str(&*format!("{} ", col)); col += 1; }
        grid.push_str("\n");
        let mut row = 0;
        while row < self.size {
            let mut col = 0;
            let blue_row = self.blues.main[row];
            let red_row = self.reds.main[row];
            let rock_row = self.rocks.main[row];
            grid.push_str(&*format!("{} ", row));
            while col < self.size {
                let mut val = '-';
                let mask = 1 << col;
                if blue_row & mask > 0 {
                    val = BLUE_CHAR;
                } else if red_row & mask > 0 {
                    val = RED_CHAR;
                } else if rock_row & mask > 0 {
                    val = ROCK_CHAR;
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

impl<'a> Board<'a> {
    pub fn new(size: usize, diag_lookup: &'a DiagLookup) -> Board<'a> {
        let b = Overlay::new(size, &diag_lookup);
        let rd = Overlay::new(size, &diag_lookup);
        let rk = Overlay::new(size, &diag_lookup);
        Board {
            turn: Player::Blue,
            size: size,
            blues: b,
            reds: rd,
            rocks: rk
        }
    }

    pub fn from_str(size: usize, d: &'a DiagLookup, s: &str) -> Board<'a> {
        let mut b = Self::new(size, &d);
        for (row, row_str) in s.lines().enumerate() {
            if row == 0 { continue; }
            for (col, c) in row_str.trim()[2..size * 2 + 1].replace(" ", "").chars().enumerate() {
                match c {
                    BLUE_CHAR => b.set(row - 1, col, Some(Piece::Blue)),
                    RED_CHAR => b.set(row - 1, col, Some(Piece::Red)),
                    ROCK_CHAR => b.set(row - 1, col, Some(Piece::Rock)),
                    _ => {},
                }
            }
        }
        b
    }

    pub fn next_turn(&mut self) {
        if self.turn == Player::Blue {
            self.turn = Player::Red;
        } else {
            self.turn = Player::Blue;
        }
    }


    // Get horizontal moves, given the board masks.
    // We must call with both orthogonal board representations to get all moves.
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
        let mut row_moves = self.get_axis_moves(&self.blues.main, &self.reds.main,
                                                &self.rocks.main, false);
        let mut col_moves = self.get_axis_moves(&self.blues.invert, &self.reds.invert,
                                                &self.rocks.invert, true);
        row_moves.append(&mut col_moves);
        row_moves
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
        self.blues.clear(row, col);
        self.reds.clear(row, col);
        self.rocks.clear(row, col);
        if let Some(color) = val {
            let mut overlay_to_set = match color {
                Piece::Blue => &mut self.blues,
                Piece::Red => &mut self.reds,
                Piece::Rock => &mut self.rocks,
            };
            overlay_to_set.set(row, col);
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, row: usize, col: usize) -> Option<Piece> {
        if self.blues.get(row, col) { return Some(Piece::Blue) };
        if self.reds.get(row, col) { return Some(Piece::Red) };
        if self.rocks.get(row, col) { return Some(Piece::Rock) };
        None
    }

    // Returns whether or not current Board state is a win for `player`
    pub fn is_win_state(&self, player: Player) -> bool {
        let overlay = match player { Player::Red => &self.reds, Player::Blue => &self.blues };
        overlay.is_win_state()
    }

    // Returns difference in lengths of each player's longest contiguous run. If a player is in a
    // win state, add 8 extra points to their existing 4.
    pub fn score(&self, player: Player) -> i32 {
        let (mine, theirs) = match player {
            Player::Red => (&self.reds, &self.blues),
            Player::Blue => (&self.blues, &self.reds),
        };
        mine.score() - theirs.score()
    }
}

#[test]
fn test_get_set() {
    let d = DiagLookup::new(4);
    let mut b = Board::new(4, &d);

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
    let d = DiagLookup::new(4);
    let mut b = Board::new(4, &d);
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
    let d = DiagLookup::new(4);
    let mut b = Board::new(4, &d);

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
fn test_get_moves_basic_2() {
    let d = DiagLookup::new(2);
    let mut b = Board::new(2, &d);
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
    let d = DiagLookup::new(3);
    let mut b = Board::new(3, &d);
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
    let d = DiagLookup::new(2);
    let b = Board::new(2, &d);
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
    let d = DiagLookup::new(3);
    let b = Board::new(3, &d);
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

#[test]
fn test_board_from_str() {
    let s = "+ 0 1 2 3
             0 b - - -
             1 r - - #
             2 - - - b
             3 - - - -";
    let d = DiagLookup::new(4);
    let b = Board::from_str(4, &d, s);
    assert_eq!(Some(Piece::Blue), b.get(0, 0));
    assert_eq!(Some(Piece::Red), b.get(1, 0));
    assert_eq!(Some(Piece::Rock), b.get(1, 3));
    assert_eq!(Some(Piece::Blue), b.get(2, 3));
}

#[test]
fn test_score_blank() {
    let s = "+ 0 1 2 3 4
             0 - - - - -
             1 - - - # -
             2 - - - - -
             3 - - - - -
             4 - - - - -";
    let d = DiagLookup::new(5);
    let b = Board::from_str(5, &d, s);
    assert_eq!(b.score(Player::Blue), 0);
}

#[test]
fn test_score_even_1() {
    let s = "+ 0 1 2 3 4
             0 - - b - -
             1 - - - # -
             2 - - r - -
             3 - - - - -
             4 - - - - -";
    let d = DiagLookup::new(5);
    let b = Board::from_str(5, &d, s);
    assert_eq!(b.score(Player::Blue), 0);
}

#[test]
fn test_score_even_2() {
    let s = "+ 0 1 2 3 4
             0 - - b b -
             1 - - - # -
             2 - - r - -
             3 - - r - -
             4 - - - - -";
    let d = DiagLookup::new(5);
    let b = Board::from_str(5, &d, s);
    assert_eq!(b.score(Player::Blue), 0);
}

#[test]
fn test_score_even_3() {
    let s = "+ 0 1 2 3 4
             0 - b b b -
             1 - - - # -
             2 - - r - -
             3 - - r r -
             4 - - - - r";
    let d = DiagLookup::new(5);
    let b = Board::from_str(5, &d, s);
    assert_eq!(b.score(Player::Blue), 0);
}

#[test]
fn test_score_adv_1() {
    let s = "+ 0 1 2 3 4
             0 - b b b -
             1 - - - # -
             2 - - r - -
             3 - - r - -
             4 - - - - -";
    let d = DiagLookup::new(5);
    let b = Board::from_str(5, &d, s);
    assert_eq!(b.score(Player::Blue), 1);
}

#[test]
fn test_score_win() {
    let s = "+ 0 1 2 3 4
             0 - b b b b
             1 - - - # -
             2 - - r - -
             3 - - r - -
             4 - - - - -";
    let d = DiagLookup::new(5);
    let b = Board::from_str(5, &d, s);
    assert_eq!(b.score(Player::Blue), 10);
}

#[test]
fn test_score_lose() {
    let s = "+ 0 1 2 3 4 5 6 7
             0 - - - - - - - -
             1 - - r # - - - -
             2 - - - - - - - -
             3 - # - - r - - b
             4 b b b b # - - r
             5 - - - b b - - r
             6 - - - - - r - r
             7 - - - - - r r b";
    let d = DiagLookup::new(8);
    let b = Board::from_str(8, &d, s);
    assert_eq!(b.score(Player::Red), -9);
}
