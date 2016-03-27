extern crate core;

use self::core::cmp::{max};
use diag_lookup;
use util::*;

// Currently, score is equal to the length of the longest contiguous segment
#[inline(always)]
fn score_row(mut row: u64) -> i32 {
    if row == 0 { return 0; }
    let mut i = 1;
    while i < 4 {
        row = row & (row >> 1);
        if row == 0 { break; }
        i += 1;
    }
    i
}

#[inline(always)]
fn is_row_win(row: u64) -> bool {
    score_row(row) >= 4
}

// Stores the positions of one type of piece on a square board. Maintains four different representations
// of the board, one for each 45-degree rotation.
#[derive(Clone, Debug)]
pub struct Overlay {
    size: usize,
    pub main: [u64; BOARD_SIZE],
    pub invert: [u64; BOARD_SIZE],
    diag: [u64; BOARD_DIAG_SIZE],
    diag_rot: [u64; BOARD_DIAG_SIZE],
}

impl Overlay {
    pub fn new(size: usize) -> Overlay {
        Overlay {
            size: size,
            main: [0; BOARD_SIZE],
            invert: [0; BOARD_SIZE],
            diag: [0; BOARD_DIAG_SIZE],
            diag_rot: [0; BOARD_DIAG_SIZE],
        }
    }

    pub fn set(&mut self, row: usize, col: usize) {
        let (row_invert, col_invert) = (col, row);
        self.main[row] |= 1 << col;
        self.invert[row_invert] |= 1 << col_invert;
        let (diag_coord, diag_rot_coord) = diag_lookup::lookup(self.size, row, col);
        self.diag[diag_coord.0] |= 1 << diag_coord.1;
        self.diag_rot[diag_rot_coord.0] |= 1 << diag_rot_coord.1;
    }

    pub fn clear(&mut self, row: usize, col: usize) {
        let (row_invert, col_invert) = (col, row);
        self.main[row] &= !0 ^ (1 << col);
        self.invert[row_invert] &= !0 ^ (1 << col_invert);
        let (diag_coord, diag_rot_coord) = diag_lookup::lookup(self.size, row, col);
        self.diag[diag_coord.0] &= !0 ^ (1 << diag_coord.1);
        self.diag_rot[diag_rot_coord.0] &= !0 ^ (1 << diag_rot_coord.1);
    }

    pub fn get(&self, row: usize, col: usize) -> bool {
        self.main[row] & (1 << col) != 0
    }

    pub fn score(&self) -> i32 {
        let mut score = 0;
        for row in &self.main { score = max(score, score_row(*row)); }
        for row in &self.invert { score = max(score, score_row(*row)); }
        for row in &self.diag { score = max(score, score_row(*row)); }
        for row in &self.diag_rot { score = max(score, score_row(*row)); }
        score | ((score & 4) << 1) // boost score if score == 4
    }

    pub fn is_win_state(&self) -> bool {
        for row in &self.main { if is_row_win(*row) { return true; } }
        for row in &self.invert { if is_row_win(*row) { return true; } }
        for row in &self.diag { if is_row_win(*row) { return true; } }
        for row in &self.diag_rot { if is_row_win(*row) { return true; } }
        false
    }
}

#[test]
fn test_get_set() {
    let mut b = Overlay::new(4);
    assert_eq!(false, b.get(1, 0));
    b.set(0, 0);
    assert_eq!(true, b.get(0, 0));
    assert_eq!(false, b.get(1, 0));
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
fn test_score_row() {
    assert_eq!(0, score_row(0));
    assert_eq!(1, score_row(1));
    assert_eq!(2, score_row(0b11));
    assert_eq!(3, score_row(0b111));
    assert_eq!(4, score_row(0b1111));
    assert_eq!(4, score_row(0b11110));
    assert_eq!(4, score_row(0xF000000000000000));
    assert_eq!(4, score_row(0xAA02F20011002345));
    assert_eq!(2, score_row(0xAA55000011002345));
    assert_eq!(4, score_row(0x1E00000000000000));
    assert_eq!(3, score_row(0xE000000000000000));
}
