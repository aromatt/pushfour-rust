mod board;
use board::*;

fn main() {
    println!("Hello, world!");
    let mut b = Board::new(4);
    println!("{:?}", b.set(&Pos { x: 1, y: 1 }, Space::Blue));
    println!("{:?}", b.set(&Pos { x: 1, y: 1 }, Space::Red));
    println!("{:?}", b.get(&Pos { x: 1, y: 1 }));
    println!("{:?}", b);
}
