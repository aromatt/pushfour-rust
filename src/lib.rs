#![feature(rustc_private)]
#![feature(platform_intrinsics)]
#![feature(intrinsics)]
#![feature(asm)]

extern crate minimax;

pub mod board;

use self::minimax::Game;
use board::*;

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
        if self.player == Player::Red {
            b.score(Player::Red)
        } else {
            b.score(Player::Blue)
        }
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
