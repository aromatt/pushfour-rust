extern crate minimax;
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

impl Game<Board, Move, f32> for PushfourGame {
    fn get_moves(&self, root: &Board) -> Vec<Move> {
        root.get_moves()
    }

    fn eval(&self, state: &Board, my_turn: bool) -> f32 { // TODO my_turn not used
        if self.player == Player::Red {
            if state.is_win_state(Player::Red) {
                1.0
            } else if state.is_win_state(Player::Blue) {
                -1.0
            } else {
                0.0
            }
        } else {
            if state.is_win_state(Player::Blue) {
                1.0
            } else if state.is_win_state(Player::Red) {
                -1.0
            } else {
                0.0
            }
        }
    }

    fn apply(&self, state: &Board, m: Move) -> Board {
        let mut cloned = state.clone();
        cloned.set_move(m);
        cloned.next_turn();
        cloned
    }
}
