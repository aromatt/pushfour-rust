extern crate core;

use self::core::cmp::{max};

const BOARD_SIZE: usize = 8;
const BOARD_DIAG_SIZE: usize = BOARD_SIZE * 2 - 1;

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

#[derive(Clone, Copy, Debug)]
pub struct Coord(usize, usize);

fn rotate_cw(size: usize, row: usize, col: usize) -> (usize, usize) {
    (col, size - row - 1)
}


#[derive(Clone, Debug)]
pub struct DiagLookup {
    main: [[Coord; BOARD_SIZE]; BOARD_SIZE],
    rot: [[Coord; BOARD_SIZE]; BOARD_SIZE]
}

impl DiagLookup {
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
    pub fn new(size: usize) -> DiagLookup {
        let mut main =  [[Coord(0, 0); BOARD_SIZE]; BOARD_SIZE];
        let mut rot = [[Coord(0, 0); BOARD_SIZE]; BOARD_SIZE];
        let mut key_row_reset = 1;
        let mut key_col_reset = 1;
        let mut key_row = 0;
        let mut key_col = 0;
        let mut val_row = 0;
        let mut val_col = 0;
        let mut total = 0;
        while total < size * size {
            main[key_row][key_col] = Coord(val_row, val_col);
            let (key_row_rot, key_col_rot) = rotate_cw(size, key_row, key_col);
            rot[key_row_rot][key_col_rot] = Coord(val_row, val_col);

            // Reset from top row to the left column
            if key_row == 0 && key_row_reset < size {
                key_row = key_row_reset;
                key_col = 0;
                key_row_reset += 1;
                val_col = 0;
                val_row += 1;
                // Reset from the right column to the bottom row
            } else if key_col == size - 1 && key_col_reset < size {
                key_col = key_col_reset;
                key_row = size - 1;
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
        DiagLookup {
            main: main,
            rot: rot,
        }
    }
}

// Stores the positions of one type of piece on a square board. Maintains four different representations
// of the board, one for each 45-degree rotation.
#[derive(Clone, Debug)]
pub struct Overlay<'a> {
    size: usize,
    pub main: [u64; BOARD_SIZE],
    pub invert: [u64; BOARD_SIZE],
    diag: [u64; BOARD_DIAG_SIZE],
    diag_rot: [u64; BOARD_DIAG_SIZE],
    pub diag_lookup: &'a DiagLookup,
}

impl<'a> Overlay<'a> {
    pub fn new(size: usize, diag_lookup: &'a DiagLookup) -> Overlay {
        Overlay {
            size: size,
            main: [0; BOARD_SIZE],
            invert: [0; BOARD_SIZE],
            diag: [0; BOARD_DIAG_SIZE],
            diag_rot: [0; BOARD_DIAG_SIZE],
            diag_lookup: diag_lookup,
        }
    }

    pub fn set(&mut self, row: usize, col: usize) {
        let (row_invert, col_invert) = (col, row);
        let Coord(drow, dcol) = self.diag_lookup.main[row][col];
        let Coord(drow_rot, dcol_rot) = self.diag_lookup.rot[row][col];
        self.main[row] |= 1 << col;
        self.invert[row_invert] |= 1 << col_invert;
        self.diag[drow] |= 1 << dcol;
        self.diag_rot[drow_rot] |= 1 << dcol_rot;
    }

    pub fn clear(&mut self, row: usize, col: usize) {
        let (row_invert, col_invert) = (col, row);
        let Coord(drow, dcol) = self.diag_lookup.main[row][col];
        let Coord(drow_rot, dcol_rot) = self.diag_lookup.rot[row][col];
        self.main[row] &= !0 ^ (1 << col);
        self.invert[row_invert] &= !0 ^ (1 << col_invert);
        self.diag[drow] &= !0 ^ (1 << dcol);
        self.diag_rot[drow_rot] &= !0 ^ (1 << dcol_rot);
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
    let d = DiagLookup::new(4);
    let mut b = Overlay::new(4, &d);
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
