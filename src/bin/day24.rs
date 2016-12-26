extern crate common;
#[macro_use]
extern crate itertools;
extern crate permutohedron;

use common::Problem;
use common::errors::*;
use permutohedron::LexicalPermutation;
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs::File;
use std::io::Read;
use std::str;

type Position = (usize, usize);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Cell {
    Wall,
    Open,
    Number(u8),
}

pub struct Map(HashMap<Position, Cell>);

impl Map {
    fn from_str(input: &str) -> Result<Map> {
        let map = input.lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| {
                        let cell = match c {
                            '.' => Cell::Open,
                            '#' => Cell::Wall,
                            c if c.is_digit(10) => Cell::Number(c.to_digit(10).unwrap() as u8),
                            _ => unreachable!(),
                        };
                        ((x, y), cell)
                    })
            })
            .collect::<HashMap<_, _>>();
        Ok(Map(map))
    }

    fn max_number(&self) -> Option<u8> {
        for i in 0..u8::max_value() {
            if self.number_position(i).is_none() {
                return Some(i - 1);
            }
        }

        None
    }

    fn number_position(&self, number: u8) -> Option<Position> {
        self.0
            .iter()
            .find(|&(_, &cell)| {
                match cell {
                    Cell::Number(n) if n == number => true,
                    _ => false,
                }
            })
            .map(|(position, _)| *position)
    }

    fn distance_between(&self, start: u8, end: u8) -> Option<usize> {
        self.number_position(start)
            .and_then(|start| self.number_position(end).map(|end| (start, end)))
            .and_then(|(start, end)| self.search(start, end))
    }

    fn search(&self, start: Position, end: Position) -> Option<usize> {
        let step = Step {
            position: start,
            count: 0,
        };
        let mut visited = HashSet::new();
        let mut next_steps = VecDeque::new();
        next_steps.push_back(step);

        while let Some(step) = next_steps.pop_front() {
            if visited.contains(&step.position) {
                continue;
            }

            if step.position == end {
                return Some(step.count);
            }

            for next_step in step.next_steps(self) {
                next_steps.push_back(next_step);
            }

            visited.insert(step.position);
        }

        None
    }
}

#[derive(Clone, Debug)]
struct Step {
    position: Position,
    count: usize,
}

impl Step {
    fn next_steps(&self, map: &Map) -> Vec<Step> {
        iproduct!(-1isize..2, -1isize..2)
            .filter(|&(dx, dy)| dx.abs() != dy.abs())
            .map(|(dx, dy)| (self.position.0 as isize + dx, self.position.1 as isize + dy))
            .map(|(x, y)| (x as usize, y as usize))
            .filter(|pos| {
                match map.0.get(pos) {
                    Some(&Cell::Open) |
                    Some(&Cell::Number(_)) => true,
                    _ => false,
                }
            })
            .map(|pos| {
                let mut step = self.clone();
                step.position = pos;
                step.count += 1;
                step
            })
            .collect()
    }
}

fn solve(map: &Map, return_to_start: bool) {
    let max_number = map.max_number().unwrap();
    let mut data: Vec<_> = (1..(max_number + 1)).collect();
    let mut paths = Vec::new();

    loop {
        let mut path = data.clone();
        path.insert(0, 0);
        if return_to_start {
            path.push(0);
        }

        paths.push(path);
        if !data.next_permutation() {
            break;
        }
    }

    let answer = paths.iter()
        .map(|path| {
            path.windows(2)
                .map(|parts| map.distance_between(parts[0], parts[1]).unwrap())
                .sum::<usize>()
        })
        .min()
        .unwrap();
    println!("Answer: {}", answer);
}

fn run() -> Result<()> {
    let problem = Problem::from_arg(1)?;
    let filename = env::args().nth(2).expect("Must provide filename");
    let mut file = File::open(filename)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    let map = Map::from_str(&buffer)?;

    match problem {
        Problem::Part1 => solve(&map, false),
        Problem::Part2 => solve(&map, true),
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
