#![feature(rustc_private)]
#![feature(platform_intrinsics)]
#![feature(intrinsics)]

extern crate time;

use std::io;
use board::*;
use pushfour_game::PushfourGame;
use pushfour_game::minimax::{Minimax, Game};
use time::*;

mod board;
mod pushfour_game;

static BOARD_SIZE: usize = 4;
static DEPTH: i32 = 2;

// TIME (2 rocks):
//   size | depth | first turn time (sec)
//   4      6       5
//   5      5       8
//   5      6       85
//   6      4       2.2
//   6      5       33
//   7      4       5.7
//   8      3       0.5
//   8      4       13
//
// Clean up player... I don't think Board needs it
fn main() {
    let g = PushfourGame::new(Player::Red);
    let mut b = Board::new(BOARD_SIZE);
    println!("New pushfour game.");
    b.set(2, 1, Some(Piece::Rock));
    b.set(0, 3, Some(Piece::Rock));
    //b.set(4, 4, Some(Piece::Rock));
    println!("Board state: {:?}", b);

    let mut timer;
    loop {
        if b.get_moves_set().len() == 0 {
            println!("\nCat's game.\n");
            break;
        }
        let mut human_input = String::new();
        println!("Your turn!");
        io::stdin().read_line(&mut human_input)
            .ok()
            .expect("Failed reading input");
        let coords: Vec<_> = human_input.trim()
            .split(":")
            .map(|c| c.parse().unwrap())
            .collect();

        let human_move = Move { row: coords[0], col: coords[1], player: Player::Blue };
        if !b.get_moves_set().contains(&human_move) {
            println!("Unavailable move! {:?}", human_move);
            continue;
        }
        b = g.apply(&b, human_move);

        println!("Board state: {:?}", b);

        if b.is_win_state(Player::Blue) {
            println!("\nYou win!\n");
            break;
        }
        timer = now();
        let bot_move = Minimax::best_move(DEPTH, &g, &b);
        println!("My move: {:?} ({:?} ms)", bot_move, (now() - timer).num_milliseconds());
        b = g.apply(&b, bot_move);
        println!("New state: {:?}", b);

        if b.is_win_state(Player::Red) {
            println!("\nI win!\n");
            break;
        }
    }
}
