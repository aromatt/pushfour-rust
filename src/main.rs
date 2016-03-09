#![feature(rustc_private)]
#![feature(platform_intrinsics)]
#![feature(intrinsics)]
#![feature(asm)]

use std::io;
use board::*;
use pushfour_game::PushfourGame;
use pushfour_game::minimax::{Minimax, Game};

mod board;
mod pushfour_game;

static BOARD_SIZE: usize = 6;
static DEPTH: i32 = 5;

// Clean up player... I don't think Board needs it
fn main() {
    let g = PushfourGame::new(Player::Red);
    let mut b = Board::new(BOARD_SIZE);
    println!("New pushfour game.");
    b.set(1, 3, Some(Piece::Rock));
    b.set(3, 1, Some(Piece::Rock));
    b.set(4, 4, Some(Piece::Rock));
    println!("Board state: {:?}", b);

    loop {
        if b.get_moves_set().len() == 0 {
            println!("\nCat's game.\n");
            break;
        }

        // Wait for human player to move
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

        // Apply human move
        b = g.apply(&b, human_move);
        println!("Board state: {:?}", b);
        if b.is_win_state(Player::Blue) {
            println!("\nYou win!\n");
            break;
        }

        if b.get_moves_set().len() == 0 {
            println!("\nCat's game.\n");
            break;
        }

        // Compute and apply bot move
        let bot_move = Minimax::best_move(DEPTH, &g, &b);
        println!("My move: {:?}", bot_move);
        b = g.apply(&b, bot_move);
        println!("New state: {:?}", b);
        if b.is_win_state(Player::Red) {
            println!("\nI win!\n");
            break;
        }
    }
}
