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

    fn eval(&self, state: &Board, _: bool) -> i32 {
        if self.player == Player::Red {
            if state.is_win_state(Player::Red) {
                1
            } else if state.is_win_state(Player::Blue) {
                -1
            } else {
                0
            }
        } else {
            if state.is_win_state(Player::Blue) {
                1
            } else if state.is_win_state(Player::Red) {
                -1
            } else {
                0
            }
        }
    }

    fn gameover(&self, state: &Board) -> bool {
        state.is_win_state(Player::Blue) || state.is_win_state(Player::Red)
    }

    fn apply(&self, state: &Board, m: Move) -> Board {
        let mut cloned = state.clone();
        cloned.set_move(m);
        cloned.next_turn();
        cloned
    }
}
