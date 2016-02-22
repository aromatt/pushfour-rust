extern crate minimax;
use self::minimax::Game;
use board::*;

pub struct PushfourGame;

impl Game<Board, Move, bool> for PushfourGame {
    fn get_moves(&self, root: &Board) -> Vec<Move> {
        let mut moves = Vec::new();
        moves
    }

    fn eval(&self, state: &Board, my_turn: bool) -> bool {
        if true {
            if my_turn {
                false
            } else {
                true
            }
        } else {
            false
        }
    }

    fn apply(&self, state: &Board, m: Move) -> Board {
        let mut cloned = state.clone();
        cloned.set_move(m);
        cloned
    }
}
