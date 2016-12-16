extern crate common;
extern crate num_integer;

use common::Problem;
use common::errors::*;
use num_integer as num;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::Read;
use std::result;
use std::str;

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    West,
    South,
    East,
}

type Vector = (i16, i16);

const DIRECTIONS: [Direction; 4] =
    [Direction::North, Direction::West, Direction::South, Direction::East];

fn parse_move(part: &str) -> Result<(&str, i16)> {
    let turn = &part[0..1];
    let length = part[1..].parse().chain_err(|| "unable to parse");
    length.map(|l| (turn, l))
}

fn move_to_direction(current_direction: &mut i8, value: (&str, i16)) -> Option<(Direction, i16)> {
    let (turn, length) = value;
    match turn {
        "L" => {
            *current_direction = num::mod_floor(*current_direction + 1, 4);
            let direction = DIRECTIONS[*current_direction as usize];
            Some((direction, length))
        }
        "R" => {
            *current_direction = num::mod_floor(*current_direction - 1, 4);
            let direction = DIRECTIONS[*current_direction as usize];
            Some((direction, length))
        }
        _ => unimplemented!(),
    }
}

fn direction_to_vector(value: (Direction, i16)) -> Vector {
    let (direction, length) = value;
    match direction {
        Direction::North => (0, length),
        Direction::West => (-length, 0),
        Direction::South => (0, -length),
        Direction::East => (length, 0),
    }
}

fn part1(buffer: &str) -> Result<()> {
    let position = buffer.split(',')
        .map(str::trim)
        .map(parse_move)
        .filter_map(result::Result::ok)
        .scan(0i8, move_to_direction)
        .map(direction_to_vector)
        .fold((0, 0), |acc, v| {
            let (x, y) = acc;
            let (dx, dy) = v;
            (x + dx, y + dy)
        });

    let (x, y) = position;
    let blocks = x.abs() + y.abs();
    println!("Final position: {:?}", position);
    println!("Blocks traveled: {}", blocks);

    Ok(())
}

fn part2(buffer: &str) -> Result<()> {
    let mut position: Vector = (0, 0);
    let mut visited = HashSet::new();
    let steps = buffer.split(',')
        .map(str::trim)
        .map(parse_move)
        .filter_map(result::Result::ok)
        .scan(0i8, move_to_direction);
    for (direction, length) in steps {
        for _ in 0..length {
            position = match direction {
                Direction::North => (position.0, position.1 + 1),
                Direction::West => (position.0 - 1, position.1),
                Direction::South => (position.0, position.1 - 1),
                Direction::East => (position.0 + 1, position.1),
            };

            if visited.contains(&position) {
                let (x, y) = position;
                let blocks = x.abs() + y.abs();
                println!("Crossed twice: {:?}", position);
                println!("Blocks traveled: {}", blocks);

                return Ok(());
            } else {
                visited.insert(position);
            }
        }
    }

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
