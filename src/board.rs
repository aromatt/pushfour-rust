#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Space {
    Empty,
    Red,
    Blue,
}

#[derive(Debug)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct Board {
    size: usize,
    spaces: Vec<Vec<Space>>,
}

impl Board {
    pub fn new(size: usize) -> Self {
        let mut spaces = Vec::with_capacity(size);
        for _ in 0..size {
            let mut row = Vec::with_capacity(size);
            for _ in 0..size {
                row.push(Space::Empty);
            }
            spaces.push(row);
        }
        Board {
            size: size,
            spaces: spaces,
        }
    }

    pub fn set(&mut self, p: &Pos, val: Space) -> bool {
        if p.x < self.size && p.y < self.size {
            self.spaces[p.x][p.y] = val;
            true
        } else { false }
    }

    // TODO maybe should try to do this without copying
    pub fn get(&self, p: &Pos) -> Option<Space> {
        match self.spaces.get(p.x) {
            Some(row) => row.get(p.y).and_then(|s| Some(s.clone())),
            None => None,
        }
    }
}

#[test]
fn test_get_set() {
    let mut b = Board::new(5);
    b.set(&Pos { x: 1, y: 1 }, Space::Red);
    assert_eq!(b.get(&Pos { x: 1, y: 1 }).unwrap(), Space::Red);
}

#[test]
fn test_get_reset() {
    let mut b = Board::new(5);
    b.set(&Pos { x: 1, y: 1 }, Space::Red);
    b.set(&Pos { x: 1, y: 1 }, Space::Blue);
    assert_eq!(b.get(&Pos { x: 1, y: 1 }).unwrap(), Space::Blue);
    assert_eq!(b.get(&Pos { x: 1, y: 6 }), None);
}

#[test]
fn test_get_out_of_bounds() {
    let b = Board::new(5);
    assert_eq!(b.get(&Pos { x: 1, y: 6 }), None);
}
