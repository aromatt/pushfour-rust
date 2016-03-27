use std::fmt;
use util::*;
use util::core::cmp::{max};
use diag_lookup;

// Returns min(longest length contiguous segment of 1's with `row`, `limit`)
fn score_row_limit(mut row: u64, limit: usize) -> i32 {
    if row == 0 { return 0; }
    let mut i: i32 = 1;
    while i < limit as i32 {
        row = row & (row >> 1);
        if row == 0 { break; }
        i += 1;
    }
    i
}

// Returns the length of the longest contiguous segment of 4 or less
#[inline(always)]
fn score_row(row: u64) -> i32 {
    score_row_limit(row, 4)
}

// Returns the length of the longest segment that would be >= 4 long after ORing with `mask`.
// Returns 0 if there is no available win in `row | mask`.
#[inline(always)]
fn score_row_mask(row: u64, mask: u64, size: usize) -> i32 {
    if row == 0 { return 0; }
    let combined = row | mask;
    let combined_score = score_row_limit(combined, size);
    if combined_score < 4 { return 0; }
    let mask_score = score_row_limit(mask, size);

    // For this to be true, the row 1's must touch the mask 1's.
    if combined_score > mask_score {
        return score_row(row);
    }
    0
}

#[inline(always)]
fn is_row_win(row: u64) -> bool {
    score_row(row) >= 4
}

#[inline(always)]
fn reachable_in_row(row: u64) -> u64 {
    let mut reach = 0;
    if let Some(l) = leading_zero_idx(row) {
        reach = !0;
        if l < 63 {
            reach &= !((1 << (63 - l)) - 1);
        }
    }
    if let Some(t) = trailing_zero_idx(row) {
        if t < 63 {
            reach |= (1 << (t + 1)) - 1;
        }
    }
    reach
}

// Stores the positions of one type of piece on a square board. Maintains four different representations
// of the board, one for each 45-degree rotation.
#[derive(Clone)]
pub struct Overlay {
    size: usize,
    pub main: [u64; BOARD_SIZE],
    pub invert: [u64; BOARD_SIZE],
    diag: [u64; BOARD_DIAG_SIZE],
    diag_rot: [u64; BOARD_DIAG_SIZE],
}

impl fmt::Debug for Overlay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut grid = String::new();
        grid.push_str("\n+ ");
        let mut col = 0;
        while col < self.size { grid.push_str(&*format!("{} ", col)); col += 1; }
        grid.push_str("\n");
        let mut row = 0;
        while row < self.size {
            let mut col = 0;
            let row_bits = self.main[row];
            grid.push_str(&*format!("{} ", row));
            while col < self.size {
                let mut val = '-';
                let mask = 1 << col;
                if row_bits & mask > 0 {
                    val = ROCK_CHAR;
                }
                grid.push_str(&*format!("{} ", &val));
                col += 1;
            }
            grid.push_str("\n");
            row += 1;
        }
        write!(f, "{}", grid)
    }
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

    pub fn reachable(&self) -> Overlay {
        let mut o = Overlay::new(self.size);
        for row in 0..self.size {
            let r = reachable_in_row(self.main[row]);
            for col in 0..self.size {
                if (r & (1 << col)) > 0 { o.set(row, col); }
            }
        }
        for col in 0..self.size {
            let r = reachable_in_row(self.invert[col]);
            for row in 0..self.size {
                if r & (1 << row) > 0 { o.set(row, col); }
            }
        }
        o
    }

    pub fn merge(&mut self, other: &Overlay) {
        for (i, row) in self.main.iter_mut().enumerate() { *row |= other.main[i]; }
        for (i, row) in self.invert.iter_mut().enumerate() { *row |= other.invert[i]; }
        for (i, row) in self.diag.iter_mut().enumerate() { *row |= other.diag[i]; }
        for (i, row) in self.diag_rot.iter_mut().enumerate() { *row |= other.diag_rot[i]; }
    }

    pub fn score(&self) -> i32 {
        let mut score = 0;
        for row in &self.main { score = max(score, score_row(*row)); }
        for row in &self.invert { score = max(score, score_row(*row)); }
        for row in &self.diag { score = max(score, score_row(*row)); }
        for row in &self.diag_rot { score = max(score, score_row(*row)); }
        score | ((score & 4) << 1) // boost score if score == 4
    }

    pub fn score_with_mask(&self, mask: &Overlay) -> i32 {
        let mut score = 0;
        for (i, row) in self.main.iter().enumerate() {
            score = max(score, score_row_mask(*row, mask.main[i], self.size));
        }
        for (i, row) in self.invert.iter().enumerate() {
            score = max(score, score_row_mask(*row, mask.invert[i], self.size));
        }
        for (i, row) in self.diag.iter().enumerate() {
            score = max(score, score_row_mask(*row, mask.diag[i], self.size));
        }
        for (i, row) in self.diag_rot.iter().enumerate() {
            score = max(score, score_row_mask(*row, mask.diag_rot[i], self.size));
        }
        score | ((score & 4) << 1) // boost score if score == 4
    }

    // Faster than just checking if the total board score is > ROW_WIN_SCORE
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

#[test]
fn test_score_row_mask() {
    assert_eq!(0, score_row_mask(0, 0, 4));
    assert_eq!(2, score_row_mask(0b0110, 0b1001, 4));
    assert_eq!(0, score_row_mask(0b0110, 0b0000, 4));
    assert_eq!(2, score_row_mask(0b11, 0b1100, 4));
    assert_eq!(3, score_row_mask(0b111, 0b1000, 4));
    assert_eq!(0, score_row_mask(0b11, 0b11000, 4));
    assert_eq!(4, score_row_mask(0b1111, 0b0, 4));
    assert_eq!(0, score_row_mask(0b11, 0b1111000, 4));
    assert_eq!(1, score_row_mask(0b1000, 0b0111, 4));
}

#[test]
fn test_overlay_merge() {
    let mut o = Overlay::new(3);
    o.set(1, 2);
    assert!(o.get(1, 2));

    let mut o1 = Overlay::new(3);
    o1.set(1, 0);
    o.merge(&o1);
    assert!(o.get(1, 0));
    assert!(o.get(1, 2));
    assert!(!o.get(1, 1));
}

#[test]
fn test_reachable_in_row() {
    assert_eq!(!0, reachable_in_row(0));
    assert_eq!(0, reachable_in_row(!0));
    assert_eq!(0xFFFFFFFFFFFFFFF9, reachable_in_row(0b0110));
}


#[test]
fn test_overlay_reachable_0() {
    let mut o = Overlay::new(3);
    o.set(0, 0);
    o.set(0, 2);
    println!("{:?}", o);
    let r = o.reachable();
    println!("{:?}", r);
    assert!(r.get(0, 1));
    assert!(r.get(1, 1));
    assert!(r.get(2, 2));
}

#[test]
fn test_overlay_reachable_1() {
    let mut o = Overlay::new(3);
    o.set(0, 1);
    o.set(1, 2);
    o.set(2, 1);
    o.set(1, 0);
    println!("{:?}", o);
    let r = o.reachable();
    println!("{:?}", r);
    assert!(r.get(0, 0));
    assert!(r.get(2, 2));
    assert!(r.get(0, 2));
    assert!(r.get(2, 0));
}
