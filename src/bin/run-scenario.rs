extern crate core;
extern crate pushfour;
extern crate minimax;
extern crate regex;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use std::io;
use regex::Regex;
use core::num;

use pushfour::board::{Board, Player};
use pushfour::PushfourGame;
use minimax::{Minimax, Game};

#[derive(Debug)]
pub enum CliError {
    Io(io::Error),
    Regex(regex::Error),
    Parse(num::ParseIntError),
    InvalidName,
    NotFound,
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}

impl From<regex::Error> for CliError {
    fn from(err: regex::Error) -> CliError {
        CliError::Regex(err)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(err: num::ParseIntError) -> CliError {
        CliError::Parse(err)
    }
}

fn parse_scenario_path(path: &str) -> Result<(usize, i32), CliError> {
    let re = Regex::new(r"size_(\d*)_depth_(\d*)\.txt$").unwrap();
    let m = try!(re.captures_iter(path).nth(0).ok_or(CliError::InvalidName));
    let size = try!(m.at(1).unwrap().parse::<usize>());
    let depth = try!(m.at(2).unwrap().parse::<i32>());
    Ok((size, depth))
}

fn load_scenario(size: usize, path: &str) -> Board {
    let path = Path::new(path);
    let display = path.display();
    let mut f = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => {},
    }
    Board::from_str(size, &s)
}

fn run_scenario(path: &str) {
    let (size, depth) = parse_scenario_path(path).unwrap();
    let g = PushfourGame::new(Player::Red);
    let mut b = load_scenario(size, path);
    b.next_turn();

    print!("\n##### Scenario (depth: {}) #####{:?}\n", depth, b);
    let mv = Minimax::best_move(depth, &g, &b);
    let b_next = g.apply(&b, mv);
    println!("\nBest move:{:?}", b_next);
}

fn main() {
    let mut args = env::args();
    args.next();
    for a in args { run_scenario(&a); }
}
