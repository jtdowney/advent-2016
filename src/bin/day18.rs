extern crate common;

use common::Problem;
use common::errors::*;
use std::env;
use std::fs::File;
use std::io::Read;
use std::str;
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Safe,
    Trap,
}

struct Row(Vec<Tile>);

impl FromStr for Row {
    type Err = Error;
    fn from_str(input: &str) -> Result<Row> {
        let tiles = input.chars()
            .map(|c| {
                match c {
                    '^' => Tile::Trap,
                    '.' => Tile::Safe,
                    _ => unreachable!(),
                }
            })
            .collect();
        Ok(Row(tiles))
    }
}

impl Row {
    fn next_row(&self) -> Row {
        let tiles = self.0
            .iter()
            .enumerate()
            .map(|(i, t)| {
                let center = *t;
                let left = if i > 0 { self.0[i - 1] } else { Tile::Safe };
                let right = if i < self.0.len() - 1 {
                    self.0[i + 1]
                } else {
                    Tile::Safe
                };

                match (left, center, right) {
                    (Tile::Trap, Tile::Trap, Tile::Safe) |
                    (Tile::Safe, Tile::Trap, Tile::Trap) |
                    (Tile::Trap, Tile::Safe, Tile::Safe) |
                    (Tile::Safe, Tile::Safe, Tile::Trap) => Tile::Trap,
                    _ => Tile::Safe,
                }
            })
            .collect();
        Row(tiles)
    }
}

fn solve(input: &str, count: usize) -> Result<()> {
    let mut rows = Vec::with_capacity(count);
    let row = Row::from_str(input.trim())?;
    rows.push(row);

    for _ in 1..count {
        let row = rows.last().unwrap().next_row();
        rows.push(row);
    }

    let count = rows.iter()
        .flat_map(|row| row.0.iter().filter(|t| **t == Tile::Safe))
        .count();
    println!("Count {}", count);

    Ok(())
}

fn run() -> Result<()> {
    let problem = Problem::from_arg(1)?;
    let filename = env::args().nth(2).expect("Must provide filename");
    let mut file = File::open(filename)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    match problem {
        Problem::Part1 => solve(&buffer, 40)?,
        Problem::Part2 => solve(&buffer, 400000)?,
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
