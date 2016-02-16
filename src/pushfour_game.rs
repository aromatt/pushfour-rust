extern crate minimax;
use self::minimax::Game;
use board::*;


pub struct PushfourGame;
impl Game<i32, i32, f32> for PushfourGame {
    fn get_moves(&self, root: &i32) -> Vec<i32> {
        let mut moves = Vec::new();
        if *root < 30 { moves.push(1); }
        if *root < 29 { moves.push(2); }
        if *root < 28 { moves.push(3); }
        if *root < 26 { moves.push(5); }
        moves
    }

    fn eval(&self, state: &i32, my_turn: bool) -> f32 {
        if *state == 30 {
            if my_turn {
                -1.0
            } else {
                1.0
            }
        } else {
            0.0
        }
    }

    fn apply(&self, state: &i32, m: &i32) -> i32 {
        *state + *m
    }
}
