extern crate common;

use common::Problem;
use common::errors::*;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Coord = (i8, i8);
const INITIAL_COORD: Coord = (1, 1);

mod part1 {
    use super::Coord;
    pub fn keypad(coord: Coord) -> Option<char> {
        match coord {
            (0, 0) => Some('1'),
            (1, 0) => Some('2'),
            (2, 0) => Some('3'),
            (0, 1) => Some('4'),
            (1, 1) => Some('5'),
            (2, 1) => Some('6'),
            (0, 2) => Some('7'),
            (1, 2) => Some('8'),
            (2, 2) => Some('9'),
            _ => None,
        }
    }
}

mod part2 {
    use super::Coord;
    pub fn keypad(coord: Coord) -> Option<char> {
        match coord {
            (2, 0) => Some('1'),
            (1, 1) => Some('2'),
            (2, 1) => Some('3'),
            (3, 1) => Some('4'),
            (0, 2) => Some('5'),
            (1, 2) => Some('6'),
            (2, 2) => Some('7'),
            (3, 2) => Some('8'),
            (4, 2) => Some('9'),
            (1, 3) => Some('A'),
            (2, 3) => Some('B'),
            (3, 3) => Some('C'),
            (2, 4) => Some('D'),
            _ => None,
        }
    }
}

fn process_move<F>(coord: Coord, direction: char, keypad: &F) -> Coord
    where F: Fn(Coord) -> Option<char>
{
    let next_coord = match direction {
        'U' => (coord.0, coord.1 - 1),
        'D' => (coord.0, coord.1 + 1),
        'L' => (coord.0 - 1, coord.1),
        'R' => (coord.0 + 1, coord.1),
        _ => unreachable!(),
    };

    match keypad(next_coord) {
        Some(_) => next_coord,
        None => coord,
    }
}

fn process_line<F>(initial_coord: Coord, line: String, keypad: &F) -> Coord
    where F: Fn(Coord) -> Option<char>
{
    line.chars().fold(initial_coord, |coord, c| process_move(coord, c, keypad))
}

fn run() -> Result<()> {
    let problem = Problem::from_arg(1)?;
    let filename = env::args().nth(2).expect("Must provide filename");
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let keypad = match problem {
        Problem::Part1 => part1::keypad,
        Problem::Part2 => part2::keypad,
    };

    let answer = reader.lines()
        .filter_map(|line| line.ok())
        .scan(INITIAL_COORD, |last_coord, line| {
            *last_coord = process_line(*last_coord, line, &keypad);
            Some(*last_coord)
        })
        .map(|coord| keypad(coord).unwrap())
        .collect::<String>();
    println!("Code: {}", answer);

    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);
        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}
