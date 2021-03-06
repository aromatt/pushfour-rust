use std::fmt;
use std::collections::HashSet;
use overlay::Overlay;
use util::*;

// Representation of a pushfour board.
// It's implemented as a composition of Overlays, adding logic for getting and applying available
// moves, and some other necessities for tracking the state of the game.
#[derive(Clone)]
pub struct Board {
    size: usize,
    turn: Player,
    blues: Overlay,
    reds: Overlay,
    rocks: Overlay,
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut grid = String::new();
        grid.push_str("\n+ ");
        let mut col = 0;
        while col < self.size { grid.push_str(&*format!("{} ", col)); col += 1; }
        grid.push_str("\n");
        let mut row = 0;
        while row < self.size {
            let mut col = 0;
            let blue_row = self.blues.main[row];
            let red_row = self.reds.main[row];
            let rock_row = self.rocks.main[row];
            grid.push_str(&*format!("{} ", row));
            while col < self.size {
                let mut val = '-';
                let mask = 1 << col;
                if blue_row & mask > 0 {
                    val = BLUE_CHAR;
                } else if red_row & mask > 0 {
                    val = RED_CHAR;
                } else if rock_row & mask > 0 {
                    val = ROCK_CHAR;
                }
                grid.push_str(&*format!("{} ", &val));
                col += 1;
            }
            grid.push_str("\n");
            row += 1;
        }
        write!(f, "{}Turn: {}", grid, self.turn)
    }
}

impl Board {
    pub fn new(size: usize) -> Board {
        Board {
            turn: Player::Blue,
            size: size,
            blues: Overlay::new(size),
            reds: Overlay::new(size),
            rocks: Overlay::new(size)
        }
    }

    pub fn from_str(s: &str) -> Board {
        let size = s.lines().count() - 1;
        let mut b = Self::new(size);
        for (row, row_str) in s.lines().enumerate() {
            if row == 0 { continue; }
            for (col, c) in row_str.trim()[2..size * 2 + 1].replace(" ", "").chars().enumerate() {
                match c {
                    BLUE_CHAR => b.set(row - 1, col, Some(Piece::Blue)),
                    RED_CHAR => b.set(row - 1, col, Some(Piece::Red)),
                    ROCK_CHAR => b.set(row - 1, col, Some(Piece::Rock)),
                    _ => {},
                }
            }
        }
        b
    }

    pub fn next_turn(&mut self) {
        if self.turn == Player::Blue {
            self.turn = Player::Red;
        } else {
            self.turn = Player::Blue;
        }
    }

    // Get horizontal moves, given the board masks.
    // We must call with both orthogonal board representations to get all moves.
    fn get_axis_moves(&self, reds: &[u64], blues: &[u64],
                      rocks: &[u64], transpose: bool) -> Vec<Move> {
        let mut moves = Vec::new();
        let mut row = 0;
        while row < self.size {
            let combined = blues[row] | reds[row] | rocks[row];
            if let Some(zeros) = leading_zero_idx(combined) {
                let col = 63 - zeros;
                if col < self.size {
                    moves.push(Move {
                        row: if transpose { col } else { row },
                        col: if transpose { row } else { col },
                        player: self.turn.clone()
                    });
                }
            }
            if let Some(zeros) = trailing_zero_idx(combined) {
                let col = if zeros > self.size { Some(self.size - 1) }
                          else if zeros < self.size { Some(zeros) }
                          else { None };
                if let Some(c) = col {
                    moves.push(Move {
                        row: if transpose { c } else { row },
                        col: if transpose { row } else { c },
                        player: self.turn.clone()
                    });
                }
            }
            row += 1;
        }
        moves
    }

    // Get all moves, allowing duplicates
    pub fn get_moves_dirty(&self) -> Vec<Move> {
        let mut row_moves = self.get_axis_moves(&self.blues.main, &self.reds.main,
                                                &self.rocks.main, false);
        let mut col_moves = self.get_axis_moves(&self.blues.invert, &self.reds.invert,
                                                &self.rocks.invert, true);
        row_moves.append(&mut col_moves);
        row_moves
    }

    // Get all moves, in a HashSet
    pub fn get_moves_set(&self) -> HashSet<Move> {
        vec_to_set(&mut self.get_moves_dirty())
    }

    // Get all moves, as a uniq'd Veq
    pub fn get_moves(&self) -> Vec<Move> {
        set_to_vec(&mut self.get_moves_set())
    }

    pub fn set_move(&mut self, m: Move) {
        self.set(m.row, m.col, Some(m.player.to_piece()));
    }

    pub fn set(&mut self, row: usize, col: usize, val: Option<Piece>) {
        self.blues.clear(row, col);
        self.reds.clear(row, col);
        self.rocks.clear(row, col);
        if let Some(color) = val {
            let mut overlay_to_set = match color {
                Piece::Blue => &mut self.blues,
                Piece::Red => &mut self.reds,
                Piece::Rock => &mut self.rocks,
            };
            overlay_to_set.set(row, col);
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, row: usize, col: usize) -> Option<Piece> {
        if self.blues.get(row, col) { return Some(Piece::Blue) };
        if self.reds.get(row, col) { return Some(Piece::Red) };
        if self.rocks.get(row, col) { return Some(Piece::Rock) };
        None
    }

    // Returns whether or not current Board state is a win for `player`
    pub fn is_win_state(&self, player: Player) -> bool {
        let overlay = match player { Player::Red => &self.reds, Player::Blue => &self.blues };
        overlay.is_win_state()
    }

    // Returns difference in lengths of each player's longest contiguous run. If a player is in a
    // win state, add 8 extra points to their existing 4.
    pub fn score(&self, player: Player) -> i32 {
        let (mine, theirs) = match player {
            Player::Red => (&self.reds, &self.blues),
            Player::Blue => (&self.blues, &self.reds),
        };
        mine.score() - theirs.score()
    }

    // Returns an Overlay that contains all currently-populated coordinates.
    // TODO pretty inefficient. Merging Overlays is slow because it has to fill in
    // the diagonals; can't just OR things together. This would be faster if we did
    // it by ORing things together, then filled in the diagonals after.
    pub fn populated(&self) -> Overlay {
        let mut populated = self.rocks.clone();
        populated.merge(&self.blues);
        populated.merge(&self.reds);
        populated
    }

    // Returns an Overlay that contains all reachable (unpopulated) coordinates.
    pub fn reachable(&self) -> Overlay {
        self.populated().reachable()
    }

    // Like score(), but considers reachability of wins. For a segment to count, there needs to be
    // a reachable win containing that segment.
    pub fn score_reachable(&self, player: Player) -> i32 {
        let (mine, theirs) = match player {
            Player::Red => (&self.reds, &self.blues),
            Player::Blue => (&self.blues, &self.reds),
        };
        let r = self.reachable();
        mine.score_with_mask(&r) - theirs.score_with_mask(&r)
    }
}

#[test]
fn test_get_set() {
    let mut b = Board::new(4);

    assert_eq!(None, b.get(1, 0));

    b.set(0, 0, Some(Piece::Blue));
    assert_eq!(Some(Piece::Blue), b.get(0, 0));

    assert_eq!(None, b.get(1, 0));

    b.set(0, 0, Some(Piece::Red));
    assert_eq!(Some(Piece::Red), b.get(0, 0));

    b.set(0, 0, None);
    assert_eq!(None, b.get(0, 0));

    b.set(0, 0, Some(Piece::Rock));
    assert_eq!(Some(Piece::Rock), b.get(0, 0));
}

#[test]
fn test_get_set_row() {
    let mut b = Board::new(4);
    b.set(0, 0, Some(Piece::Blue));
    assert_eq!(Some(Piece::Blue), b.get(0, 0));

    // Verify setting red in the same row doesn't clear blue
    b.set(0, 1, Some(Piece::Red));
    assert_eq!(Some(Piece::Blue), b.get(0, 0));
    assert_eq!(Some(Piece::Red), b.get(0, 1));

    // Again
    b.set(0, 2, Some(Piece::Rock));
    assert_eq!(Some(Piece::Blue), b.get(0, 0));
    assert_eq!(Some(Piece::Rock), b.get(0, 2));

}

#[test]
fn test_clone() {
    let mut b = Board::new(4);

    assert_eq!(None, b.get(0, 0));

    b.set(0, 0, Some(Piece::Blue));
    assert_eq!(Some(Piece::Blue), b.get(0, 0));

    let mut c = b.clone();
    assert_eq!(Some(Piece::Blue), c.get(0, 0));

    // Change c and makes sure b is unchanged
    c.set(0, 0, Some(Piece::Red));
    assert_eq!(Some(Piece::Blue), b.get(0, 0));
}

#[test]
fn test_get_moves_basic_2() {
    let mut b = Board::new(2);
    b.set(0, 0, Some(Piece::Blue)); // B B
    b.set(0, 1, Some(Piece::Blue)); // 0 0
    let mut expected = vec![
        Move { row: 1, col: 0, player: Player::Blue },
        Move { row: 1, col: 1, player: Player::Blue },
    ];
    assert_eq!(vec_to_set(&mut expected), vec_to_set(&mut b.get_moves()));
}

#[test]
fn test_get_moves_basic_3() {
    let mut b = Board::new(3);
    b.set(0, 0, Some(Piece::Blue)); // B 0 0
    b.set(1, 1, Some(Piece::Blue)); // 0 B 0
    b.set(2, 2, Some(Piece::Blue)); // 0 0 B
    let mut expected = vec![
        Move { row: 0, col: 1, player: Player::Blue },
        Move { row: 1, col: 2, player: Player::Blue },
        Move { row: 1, col: 0, player: Player::Blue },
        Move { row: 2, col: 1, player: Player::Blue },
    ];
    assert_eq!(vec_to_set(&mut expected), vec_to_set(&mut b.get_moves()));
}

#[test]
fn test_get_moves_empty_2() {
    let b = Board::new(2);
    let mut expected = vec![
        Move { row: 0, col: 0, player: Player::Blue },
        Move { row: 0, col: 1, player: Player::Blue },
        Move { row: 1, col: 0, player: Player::Blue },
        Move { row: 1, col: 1, player: Player::Blue },
    ];
    assert_eq!(vec_to_set(&mut expected), vec_to_set(&mut b.get_moves()));
}

#[test]
fn test_get_moves_empty_3() {
    let b = Board::new(3);
    let mut expected = vec![
        Move { row: 0, col: 0, player: Player::Blue },
        Move { row: 0, col: 1, player: Player::Blue },
        Move { row: 0, col: 2, player: Player::Blue },
        Move { row: 1, col: 0, player: Player::Blue },
        Move { row: 1, col: 2, player: Player::Blue },
        Move { row: 2, col: 0, player: Player::Blue },
        Move { row: 2, col: 1, player: Player::Blue },
        Move { row: 2, col: 2, player: Player::Blue },
    ];
    assert_eq!(vec_to_set(&mut expected), vec_to_set(&mut b.get_moves()));
}

#[test]
fn test_board_from_str() {
    let s = "+ 0 1 2 3
             0 b - - -
             1 r - - #
             2 - - - b
             3 - - - -";
    let b = Board::from_str(s);
    assert_eq!(Some(Piece::Blue), b.get(0, 0));
    assert_eq!(Some(Piece::Red), b.get(1, 0));
    assert_eq!(Some(Piece::Rock), b.get(1, 3));
    assert_eq!(Some(Piece::Blue), b.get(2, 3));
}

#[test]
fn test_score_blank() {
    let s = "+ 0 1 2 3 4
             0 - - - - -
             1 - - - # -
             2 - - - - -
             3 - - - - -
             4 - - - - -";
    let b = Board::from_str(s);
    assert_eq!(b.score(Player::Blue), 0);
}

#[test]
fn test_score_even_1() {
    let s = "+ 0 1 2 3 4
             0 - - b - -
             1 - - - # -
             2 - - r - -
             3 - - - - -
             4 - - - - -";
    let b = Board::from_str(s);
    assert_eq!(b.score(Player::Blue), 0);
}

#[test]
fn test_score_even_2() {
    let s = "+ 0 1 2 3 4
             0 - - b b -
             1 - - - # -
             2 - - r - -
             3 - - r - -
             4 - - - - -";
    let b = Board::from_str(s);
    assert_eq!(b.score(Player::Blue), 0);
}

#[test]
fn test_score_even_3() {
    let s = "+ 0 1 2 3 4
             0 - b b b -
             1 - - - # -
             2 - - r - -
             3 - - r r -
             4 - - - - r";
    let b = Board::from_str(s);
    assert_eq!(b.score(Player::Blue), 0);
}

#[test]
fn test_score_adv_1() {
    let s = "+ 0 1 2 3 4
             0 - b b b -
             1 - - - # -
             2 - - r - -
             3 - - r - -
             4 - - - - -";
    let b = Board::from_str(s);
    assert_eq!(b.score(Player::Blue), 1);
}

#[test]
fn test_score_win() {
    let s = "+ 0 1 2 3 4
             0 - b b b b
             1 - - - # -
             2 - - r - -
             3 - - r - -
             4 - - - - -";
    let b = Board::from_str(s);
    assert_eq!(b.score(Player::Blue), 10);
}

#[test]
fn test_score_lose() {
    let s = "+ 0 1 2 3 4 5 6 7
             0 - - - - - - - -
             1 - - r # - - - -
             2 - - - - - - - -
             3 - # - - r - - b
             4 b b b b # - - r
             5 - - - b b - - r
             6 - - - - - r - r
             7 - - - - - r r b";
    let b = Board::from_str(s);
    assert_eq!(b.score(Player::Red), -9);
}

#[test]
fn test_score_reachable_win() {
    let s = "+ 0 1 2 3 4
             0 - b b b b
             1 - - - # -
             2 - - r - -
             3 - - r - -
             4 - - - - -";
    let b = Board::from_str(s);
    assert_eq!(b.score_reachable(Player::Blue), 10);
}

#[test]
fn test_score_reachable_basic() {
    let s = "+ 0 1 2 3 4
             0 - - b b b
             1 - - - # -
             2 - - r - -
             3 - - r - -
             4 - - - - -";
    let b = Board::from_str(s);
    assert_eq!(b.score_reachable(Player::Blue), 1);
}

#[test]
fn test_score_reachable_landlock() {
    let s = "+ 0 1 2 3 4
             0 - r b b b
             1 - - - # -
             2 - - r - -
             3 - - r - -
             4 - - - - -";
    let b = Board::from_str(s);
    assert_eq!(b.score_reachable(Player::Blue), -1);
}

#[test]
fn test_score_reachable_unreachable() {
    let s = "+ 0 1 2 3 4
             0 - r - # -
             1 r - b b -
             2 - - r - -
             3 - - r - -
             4 - r - - -";
    let b = Board::from_str(s);
    assert_eq!(b.score_reachable(Player::Blue), -1);
}
