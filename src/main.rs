#![feature(rustc_private)]
#![feature(intrinsics)]

mod board;
mod pushfour_game;
use board::*;
use pushfour_game::*;
extern crate rustc_data_structures;

fn main() {
    let mut b = Board::new(3);
    b.set(0, 0, Some(Player::Blue)); // B 0 0
    b.set(1, 1, Some(Player::Blue)); // 0 B 0
    b.set(2, 2, Some(Player::Blue)); // 0 0 B
    println!("{:?}", b.get_moves());
}
