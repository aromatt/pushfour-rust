extern crate core;
extern crate pushfour;
extern crate regex;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use std::io;
use std::process;
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

fn run_scenario(path: &str, maybe_depth: Option<i32>) -> Result<bool, CliError> {
    let depth = match parse_scenario_path(path) {
        Ok(d) => d,
        Err(e) => {
            if let Some(d) = maybe_depth { d } else {
                return Err(e);
            }
        }
    };
    let g = PushfourGame::new(Player::Red);
    let mut b = load_scenario(path);
    b.next_turn();

    println!("\n##### Scenario (depth: {}) #####{:?}", depth, b);
    println!("Current board score: {}", b.score(Player::Red));
    let mv = Minimax::best_move(depth, &g, &b);
    let b_next = g.apply(&b, mv);
    println!("\nBest move:{:?}", b_next);
    println!("New board score: {}\n", b_next.score(Player::Red));
    Ok(true)
}

fn parse_args(args: &mut std::env::Args) -> Option<i32> {
    let mut args = args.peekable();
    args.next();
    if !(if let Some(ref a) = args.peek() { a == &"-d" } else { false }) { return None };
    args.next();
    if let Some(ref d) = args.next() {
        return d.parse::<i32>().ok();
    }
    None
}

fn print_usage() {
    println!("Usage:

    ./run-scenario [-d DEFAULT_DEPTH] FILE [FILE] ...

where each FILE contains depth_N in its name, unless DEFAULT_DEPTH is provided.");
}

fn main() {
    let mut args = env::args();
    let maybe_depth = parse_args(&mut args);
    for a in args {
        if !run_scenario(&a, maybe_depth).is_ok() {
            print_usage();
            process::exit(1);
        }
    }
}
