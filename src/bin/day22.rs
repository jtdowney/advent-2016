extern crate common;
#[macro_use]
extern crate itertools;

use common::Problem;
use common::errors::*;
use std::collections::BTreeMap;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::str;

type Position = (i8, i8);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Node {
    used: usize,
    available: usize,
}

impl Node {
    fn from_str(input: &str) -> Result<(Position, Node)> {
        let parts: Vec<_> = input.split_whitespace().collect();
        let used = parts[2].trim_right_matches('T').parse()?;
        let available = parts[3].trim_right_matches('T').parse()?;
        let name_parts: Vec<_> = parts[0].split('-').collect();
        let x = name_parts[1].trim_left_matches('x').parse()?;
        let y = name_parts[2].trim_left_matches('y').parse()?;
        let position = (x, y);
        Ok((position,
            Node {
                used: used,
                available: available,
            }))
    }
}

#[derive(Clone, Debug)]
struct Step {
    nodes: BTreeMap<Position, Node>,
    goal: Position,
    count: usize,
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for j in 0.. {
            for i in 0.. {
                let position = (i, j);
                let node = match self.nodes.get(&position) {
                    Some(n) => n,
                    None => {
                        if i < self.goal.0 {
                            return Ok(());
                        } else {
                            break;
                        }
                    }
                };

                if self.goal == position {
                    write!(f, "G")?;
                    continue;
                }

                if node.used == 0 {
                    write!(f, "_")?;
                } else if node.used >= 100 {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }

            writeln!(f, "")?;
        }

        Ok(())
    }
}

fn part1(input: &str) -> Result<()> {
    let nodes = input.lines().skip(2).map(Node::from_str).collect::<Result<Vec<_>>>()?;
    let pairs = iproduct!(nodes.iter(), nodes.iter())
        .filter(|&(a, b)| a != b)
        .filter(|&(&(_, a), _)| a.used != 0)
        .filter(|&(&(_, a), &(_, b))| a.used <= b.available);
    println!("Count: {}", pairs.count());
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let nodes = input.lines()
        .skip(2)
        .map(Node::from_str)
        .collect::<Result<BTreeMap<_, _>>>()?;
    let goal = *nodes.keys().filter(|&&(_, y)| y == 0).max_by_key(|&&(x, _)| x).unwrap();
    let step = Step {
        nodes: nodes,
        goal: goal,
        count: 0,
    };

    println!("{}", step);
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
