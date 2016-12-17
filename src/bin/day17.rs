extern crate common;
extern crate crypto;

use common::Problem;
use common::errors::*;
use crypto::digest::Digest;
use crypto::md5::Md5;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::Read;
use std::str;

type Coord = (i8, i8);
const START_POSITION: Coord = (0, 0);
const END_POSITION: Coord = (3, 3);
const WIDTH: i8 = 3;
const HEIGHT: i8 = 3;

#[derive(Clone, Debug)]
struct Step {
    position: Coord,
    path: String,
}

impl Step {
    fn next_steps(&self, input: &str) -> Vec<Step> {
        let mut digest = Md5::new();
        digest.input_str(input.trim());
        digest.input_str(&self.path);
        digest.result_str()
            .chars()
            .enumerate()
            .take(4)
            .filter(|&(_, c)| {
                match c {
                    'b' | 'c' | 'd' | 'e' | 'f' => true,
                    _ => false,
                }
            })
            .map(|(i, _)| {
                let position = match i {
                    0 => (self.position.0, self.position.1 - 1),
                    1 => (self.position.0, self.position.1 + 1),
                    2 => (self.position.0 - 1, self.position.1),
                    3 => (self.position.0 + 1, self.position.1),
                    _ => unreachable!(),
                };

                let path = match i {
                    0 => "U",
                    1 => "D",
                    2 => "L",
                    3 => "R",
                    _ => unreachable!(),
                };

                let mut step = self.clone();
                step.position = position;
                step.path.push_str(path);
                step
            })
            .filter(|step| step.position.0 >= 0 && step.position.1 >= 0)
            .filter(|step| step.position.0 <= WIDTH && step.position.1 <= HEIGHT)
            .collect()
    }
}

fn part1(input: &str) -> Result<()> {
    let mut next_steps = VecDeque::new();
    next_steps.push_back(Step {
        position: START_POSITION,
        path: String::new(),
    });

    while let Some(step) = next_steps.pop_front() {
        if step.position == END_POSITION {
            println!("Found path: {:?}", step.path);
            break;
        }

        for step in step.next_steps(input) {
            next_steps.push_back(step);
        }
    }

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut next_steps = VecDeque::new();
    next_steps.push_back(Step {
        position: START_POSITION,
        path: String::new(),
    });

    let mut solutions = vec![];
    while let Some(step) = next_steps.pop_front() {
        if step.position == END_POSITION {
            solutions.push(step);
            continue;
        }

        for step in step.next_steps(input) {
            next_steps.push_back(step);
        }
    }

    let longest = solutions.iter().max_by_key(|s| s.path.len());
    if let Some(step) = longest {
        println!("Longest path is {} steps", step.path.len());
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
