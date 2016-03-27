use std::io;

extern crate pushfour;
extern crate minimax;
extern crate rand;

use minimax::{Minimax, Game};
use pushfour::PushfourGame;
use pushfour::board::*;
use pushfour::util::*;
use rand::Rng;

static DEPTH: i32 = 7;
static NUM_ROCKS: u32 = 3;

// Clean up player... I don't think Board needs it
fn main() {
    let g = PushfourGame::new(Player::Red);
    let mut b = Board::new(BOARD_SIZE);
    println!("New pushfour game. Difficulty: {}", DEPTH);
    let mut rng = rand::thread_rng();
    let mut rcnt = 0;
    loop {
        let x = (rng.gen::<usize>() % BOARD_SIZE) as usize;
        let y = (rng.gen::<usize>() % BOARD_SIZE) as usize;
        if b.get(x, y).is_none() {
            b.set(x, y, Some(Piece::Rock));
            rcnt += 1;
            if rcnt == NUM_ROCKS { break; }
        }
    }

    println!("Board state: {:?}", b);

    loop {
        if b.get_moves_set().len() == 0 {
            println!("\nCat's game.\n");
            break;
        }

        // Wait for human player to move
        let mut human_input = String::new();
        io::stdin().read_line(&mut human_input)
            .ok()
            .expect("Failed reading input");
        let coords: Vec<Option<usize>> = human_input.trim()
            .split(":")
            .map(|c| c.parse::<usize>().ok())
            .collect();
        if coords[0].is_none() {
            println!("Enter coords as <row>:<col>, like '0:0'");
            continue;
        }
        let human_move = Move { row: coords[0].unwrap(), col: coords[1].unwrap(), player: Player::Blue };
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
        b = g.apply(&b, bot_move);
        println!("New state: {:?}", b);
        if b.is_win_state(Player::Red) {
            println!("\nI win!\n");
            break;
        }
    }
}
