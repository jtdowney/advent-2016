extern crate common;
extern crate itertools;

use common::errors::*;
use itertools::Itertools;
use std::collections::{BinaryHeap, BTreeSet, HashSet};
use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::str;

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
enum Part<'a> {
    Microchip(&'a str),
    Generator(&'a str),
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Floor<'a>(BTreeSet<Part<'a>>);

impl<'a> Floor<'a> {
    fn from_str(input: &'a str) -> Result<Floor<'a>> {
        let mut parts = BTreeSet::new();
        let words = input.split_whitespace().collect::<Vec<_>>();

        for (i, word) in words.iter().enumerate() {
            if let Some(part_type) = words.get(i + 1) {
                let part_type = part_type.trim_right_matches([',', '.'].as_ref());
                match part_type {
                    "generator" => {
                        parts.insert(Part::Generator(word));
                    }
                    "microchip" => {
                        let name = word.split('-').nth(0).unwrap();
                        parts.insert(Part::Microchip(name));
                    }
                    _ => continue,
                }
            }
        }

        Ok(Floor(parts))
    }

    fn is_valid(&self) -> bool {
        !self.0.iter().any(|part| {
            match *part {
                Part::Generator(_) => false,
                Part::Microchip(mname) => {
                    self.0.iter().any(|part| {
                        match *part {
                            Part::Generator(gname) => {
                                mname != gname && !self.0.contains(&Part::Generator(mname))
                            }
                            Part::Microchip(_) => false,
                        }
                    })
                }
            }
        })
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Clone, Debug)]
struct Step<'a> {
    count: usize,
    current_floor: usize,
    floors: Vec<Floor<'a>>,
}

impl<'a> Hash for Step<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.current_floor.hash(state);
        for floor in &self.floors {
            floor.hash(state);
        }
    }
}

impl<'a> PartialEq for Step<'a> {
    fn eq(&self, other: &Step<'a>) -> bool {
        self.current_floor == other.current_floor && self.floors == other.floors
    }
}

impl<'a> Eq for Step<'a> {}

impl<'a> PartialOrd for Step<'a> {
    fn partial_cmp(&self, other: &Step<'a>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Step<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score().cmp(&other.score()).reverse()
    }
}

impl<'a> Step<'a> {
    fn score(&self) -> usize {
        let distance = self.floors[0].0.len() * 5 + self.floors[1].0.len() * 2 +
                       self.floors[2].0.len();
        distance + self.count
    }

    fn is_done(&self) -> bool {
        self.floors[0].is_empty() && self.floors[1].is_empty() && self.floors[2].is_empty()
    }

    fn is_valid(&self) -> bool {
        self.floors.iter().all(Floor::is_valid)
    }

    fn next_steps(&self) -> Vec<Step<'a>> {
        let next_floors = match self.current_floor {
            0 => vec![1],
            1 => vec![0, 2],
            2 => vec![1, 3],
            3 => vec![2],
            _ => unreachable!(),
        };
        let mut next_steps = vec![];
        for next_floor in next_floors {
            for size in 1..3 {
                let part_groups = self.floors[self.current_floor]
                    .0
                    .iter()
                    .combinations(size);
                for part_group in part_groups {
                    let mut possible_step = self.clone();
                    possible_step.count += 1;
                    possible_step.current_floor = next_floor;
                    for part in part_group {
                        possible_step.floors[self.current_floor].0.remove(part);
                        possible_step.floors[next_floor].0.insert(*part);
                    }

                    if possible_step.is_valid() {
                        next_steps.push(possible_step);
                    }
                }
            }
        }

        next_steps
    }
}

fn solve(input: &str) -> Result<()> {
    let floors = input.lines().map(Floor::from_str).collect::<Result<Vec<_>>>()?;
    let step = Step {
        count: 0,
        current_floor: 0,
        floors: floors,
    };
    let mut visited_steps = HashSet::new();
    let mut next_steps = BinaryHeap::new();
    next_steps.push(step);

    while let Some(step) = next_steps.pop() {
        if step.is_done() {
            println!("Found solution in {} steps with {} visits",
                     step.count,
                     visited_steps.len());
            break;
        }

        if visited_steps.contains(&step) {
            continue;
        } else {
            visited_steps.insert(step.clone());
        }

        for next_step in step.next_steps() {
            next_steps.push(next_step);
        }
    }

    Ok(())
}

fn run() -> Result<()> {
    let filename = env::args().nth(2).expect("Must provide filename");
    let mut file = File::open(filename)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    solve(&buffer)?;

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
