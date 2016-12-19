extern crate common;
#[macro_use]
extern crate itertools;

use common::Problem;
use common::errors::*;
use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs::File;
use std::io::Read;

type Point = (u32, u32);
const STARTING_POINT: Point = (1, 1);
const ENDING_POINT: Point = (31, 39);
const MAX_STEPS: u32 = 50;

fn is_open_space(input: u32, point: Point) -> bool {
    let (x, y) = point;
    let value = x * x + 3 * x + 2 * x * y + y + y * y + input;
    value.count_ones() % 2 == 0
}

fn possible_moves(input: u32, point: Point) -> Vec<Point> {
    let (x, y) = point;
    iproduct!(-1i32..2, -1i32..2)
        .filter(|&(dx, dy)| dx == 0 || dy == 0)
        .filter(|&point| point != (0, 0))
        .map(|(dx, dy)| (x as i32 + dx, y as i32 + dy))
        .filter(|&(x, y)| x >= 0 && y >= 0)
        .map(|(x, y)| (x as u32, y as u32))
        .filter(|&point| is_open_space(input, point))
        .collect()
}

fn part1(input: &str) -> Result<()> {
    let input = input.trim().parse()?;
    let mut visited = HashSet::new();
    let mut next_moves = VecDeque::new();
    next_moves.push_back((STARTING_POINT, 0));

    while let Some((next_move, count)) = next_moves.pop_front() {
        if visited.contains(&next_move) {
            continue;
        } else {
            visited.insert(next_move);
        }

        if next_move == ENDING_POINT {
            println!("Found solution in {} steps with {} visits",
                     count,
                     visited.len());
            continue;
        }

        for possible_move in possible_moves(input, next_move) {
            next_moves.push_back((possible_move, count + 1));
        }
    }

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let input = input.trim().parse()?;
    let mut visited = HashSet::new();
    let mut next_moves = VecDeque::new();
    next_moves.push_back((STARTING_POINT, 0));

    while let Some((next_move, count)) = next_moves.pop_front() {
        if visited.contains(&next_move) {
            continue;
        } else {
            visited.insert(next_move);
        }

        if count == MAX_STEPS {
            continue;
        }

        for possible_move in possible_moves(input, next_move) {
            next_moves.push_back((possible_move, count + 1));
        }
    }

    println!("Visited {} points", visited.len());

    Ok(())
}

fn run() -> Result<()> {
    let problem = Problem::from_arg(1)?;
    let filename = env::args().nth(2).expect("Must provide filename");
    let mut file = File::open(filename)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    match problem {
        Problem::Part1 => part1(&buffer)?,
        Problem::Part2 => part2(&buffer)?,
    }

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
