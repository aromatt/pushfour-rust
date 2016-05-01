/* Adapted from rust-minimax by Helge Skogly Holm <helge.holm@gmail.com>:
 * https://github.com/deestan/rust-minimax
 * and
 * https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning#Pseudocode
 */

use std::cmp;
use std::fmt::Debug;

pub trait Game<State: Clone + Debug, Move: Copy> {
    fn get_moves(&self, &State) -> Vec<Move>;
    fn eval(&self, &State, my_turn: bool) -> i32;
    fn apply(&self, &State, Move) -> State;
    fn gameover(&self, &State) -> bool;
}

pub struct Minimax;
impl Minimax {
    pub fn best_move<State, Move, GameType>(depth: i32, game: &GameType, root: &State) -> Move
        where State: Clone + Debug,
              Move: Copy,
              GameType: Game<State, Move> {
        let (a, b) = (i32::min_value(), i32::max_value());
        let (mv, _score) = Minimax::min_max(depth, game, root, false, a, b);
        mv.expect("no moves")
    }

    fn min_max<State, Move, GameType>(depth: i32, game: &GameType, root: &State, do_min: bool,
                                      mut a: i32, mut b: i32) -> (Option<Move>, i32)
        where State: Clone + Debug,
              Move: Copy,
              GameType: Game<State, Move> {

        if depth == 5 {
            //println!("\nmin_max: {:?}\n             depth {} do_min {} a {} b {}\n",
            //         root, depth, do_min, a, b);
        }
        // Don't need to do anything if depth is 0, someone has won, or there are no moves
        if depth == 0 {
            let score = game.eval(root, do_min);
            //println!("depth is 0, returning {}", score);
            return (None, score);
        }
        if game.gameover(root) {
            let score = game.eval(root, do_min);
            //println!("gameover, returning {}", score);
            return (None, score);
        }
        let moves = game.get_moves(root);
        if moves.len() == 0 {
            let score = game.eval(root, do_min);
            //println!("no moves, returning {}", score);
            return (None, score);
        }

        let mut best_mv: Option<Move> = None;
        let mut best_v: i32 = i32::min_value();
        for &mv in moves.iter() {
            let child = game.apply(root, mv);
            let (_child_mv, child_v) = Minimax::min_max(depth - 1, game, &child, !do_min, a, b);
            if depth == 6 {
                //println!("result {} from child {:?}", child_v, child);
            }
            // If nothing else, just choose the first move
            if best_mv.is_none() {
                best_mv = Some(mv);
                best_v = child_v;
                continue;
            }

            // Otherwise, apply alpha-beta pruning
            if !do_min {
                if child_v > best_v {
                    best_v = child_v;
                    best_mv = Some(mv);
                }
                a = cmp::max(a, best_v);
                if b <= a {
                    //println!("breaking for AB depth {} a {} b {}", depth, a, b);
                    break;
                }
            } else {
                if child_v < best_v {
                    best_v = child_v;
                    best_mv = Some(mv);
                }
                b = cmp::min(b, best_v);
                if b <= a {
                    //println!("breaking for AB depth {} a {} b {}", depth, a, b);
                    break;
                }
            }
            if depth == 6 {
                //println!("At depth {}, now a {} b {}", depth, a, b);
            }
        };

        best_mv.expect("no moves?");
        if depth == 5 {
            //println!("at depth {}, returning {} for: {:?}", depth, best_v, root);
        }
        (best_mv, best_v)
    }
}

#[test]
fn it_works() {
}
