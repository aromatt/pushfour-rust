extern crate core;
extern crate pushfour;
extern crate regex;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use std::io;
use regex::Regex;
use core::num;

use pushfour::util::*;
use pushfour::board::Board;
use pushfour::PushfourGame;
use pushfour::minimax::{Minimax, Game};

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

fn parse_scenario_path(path: &str) -> Result<i32, CliError> {
    let re = Regex::new(r"depth_(\d*)\.txt$").unwrap();
    let m = try!(re.captures_iter(path).nth(0).ok_or(CliError::InvalidName));
    let depth = try!(m.at(1).unwrap().parse::<i32>());
    Ok(depth)
}

fn load_scenario(path: &str) -> Board {
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
    Board::from_str(&s)
}

fn run_scenario(path: &str) {
    let depth = parse_scenario_path(path).unwrap();
    let g = PushfourGame::new(Player::Red);
    let mut b = load_scenario(path);
    b.next_turn();

    println!("\n##### Scenario (depth: {}) #####{:?}", depth, b);
    println!("Current board score: {}", b.score(Player::Red));
    let mv = Minimax::best_move(depth, &g, &b);
    let b_next = g.apply(&b, mv);
    println!("\nBest move:{:?}", b_next);
    println!("New board score: {}\n", b_next.score(Player::Red));
}

fn main() {
    let mut args = env::args().peekable();
    args.next();
    for a in args { run_scenario(&a, ); }
}
