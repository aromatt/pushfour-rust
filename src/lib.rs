#![feature(rustc_private)]
#![feature(platform_intrinsics)]
#![feature(intrinsics)]
#![feature(asm)]

#[macro_use]
extern crate lazy_static;


pub mod overlay;
pub mod diag_lookup;
pub mod board;
pub mod minimax;
pub mod util;

use minimax::Game;
use board::*;
use util::*;

pub struct PushfourGame {
    player: Player,
}

impl PushfourGame {
    pub fn new(player: Player) -> PushfourGame {
        PushfourGame {
            player: player,
        }
    }
}

impl Game<Board, Move> for PushfourGame {
    fn get_moves(&self, root: &Board) -> Vec<Move> {
        root.get_moves()
    }

    fn eval(&self, b: &Board, _: bool) -> i32 {
        b.score_reachable(self.player)
    }

    fn gameover(&self, b: &Board) -> bool {
        b.is_win_state(Player::Blue) || b.is_win_state(Player::Red)
    }

    fn apply(&self, b: &Board, m: Move) -> Board {
        let mut cloned = b.clone();
        cloned.set_move(m);
        cloned.next_turn();
        cloned
    }
}
