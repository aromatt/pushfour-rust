extern crate rustc_data_structures;
extern crate core;

use std::fmt;
use std::fmt::Display;
use self::core::hash::Hash;
use std::collections::HashSet;

pub const BOARD_SIZE: usize = 8;

pub const BOARD_DIAG_SIZE: usize = BOARD_SIZE * 2 - 1;
pub const BLUE_CHAR: char = 'b';
pub const RED_CHAR: char = 'r';
pub const ROCK_CHAR: char = '#';

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
pub fn leading_zero_idx(x: u64) -> Option<usize> {
    unsafe {
        let i = ctlz(x) as usize;
        if i > 0 { Some(i - 1) } else { None }
    }
}

#[allow(private_no_mangle_fns)]
#[no_mangle]
pub fn trailing_zero_idx(x: u64) -> Option<usize> {
    unsafe {
        let i = cttz(x) as usize;
        if i > 0 { Some(i - 1) } else { None }
    }
}

#[inline(always)]
pub fn vec_to_set<T: Eq + Hash>(vec: &mut Vec<T>) -> HashSet<T> {
    let mut set = HashSet::new();
    for m in vec.drain(..) { set.insert(m); }
    set
}

#[inline(always)]
pub fn set_to_vec<T: Eq + Hash>(set: &mut HashSet<T>) -> Vec<T> {
    let mut vec = Vec::new();
    for m in set.drain() { vec.push(m); }
    vec
}


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

#[test]
fn test_leading_zero_idx() {
    assert_eq!(Some(63), leading_zero_idx(0));
    assert_eq!(Some(62), leading_zero_idx(1));
    assert_eq!(None, leading_zero_idx(!0));
    assert_eq!(None, leading_zero_idx(0x8000000000000000));
    assert_eq!(Some(0), leading_zero_idx(0x6000000000000000));
}

#[test]
fn test_trailing_zero_idx() {
    assert_eq!(Some(63), trailing_zero_idx(0));
    assert_eq!(None, trailing_zero_idx(1));
    assert_eq!(None, trailing_zero_idx(!0));
    assert_eq!(Some(0), trailing_zero_idx(2));
}


