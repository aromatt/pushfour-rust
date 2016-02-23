extern crate minimax;
use self::minimax::Game;
use board::*;

pub struct PushfourGame;

impl Game<Board, Move, bool> for PushfourGame {
    fn get_moves(&self, root: &Board) -> Vec<Move> {
        root.get_moves()
    }

    fn eval(&self, state: &Board, my_turn: bool) -> bool {
        // TODO
        true
    }

    fn apply(&self, state: &Board, m: Move) -> Board {
        let mut cloned = state.clone();
        cloned.set_move(m);
        cloned
    }
}
