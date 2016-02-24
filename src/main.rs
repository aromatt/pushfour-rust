#![feature(rustc_private)]
#![feature(intrinsics)]

mod board;
//mod pushfour_game;
use board::*;
//use pushfour_game::*;
extern crate rustc_data_structures;
extern crate core;

fn main() {
    let mut b = Board::new(3);
    b.set(0, 0, Some(Player::Blue)); // B 0 B
    b.set(1, 1, Some(Player::Blue)); // 0 B 0
    b.set(2, 2, Some(Player::Blue)); // 0 0 B
    b.set(0, 2, Some(Player::Blue));
    println!("{:?}", b);
    println!("{:?}", b.diag_lookup);
    println!("{:?}", b.diag_lookup_rot);

    /*
    let mut b = Board::new(4);
    println!("{:?}", b);
    println!("{:?}", b.diag_lookup);
    println!("{:?}", b.diag_lookup_r);
    */
}
