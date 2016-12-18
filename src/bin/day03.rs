extern crate common;

use common::Problem;
use common::errors::*;
use std::env;
use std::fs::File;
use std::io::Read;
use std::result;
use std::str;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
struct Triangle(u16, u16, u16);

impl FromStr for Triangle {
    type Err = Error;
    fn from_str(line: &str) -> Result<Triangle> {
        let mut parts = line.split_whitespace();
        Ok(Triangle(parts.next().unwrap().parse()?,
                    parts.next().unwrap().parse()?,
                    parts.next().unwrap().parse()?))
    }
}

impl Triangle {
    fn is_possible(&self) -> bool {
        let Triangle(x, y, z) = *self;
        (x + y) > z && (x + z) > y && (y + z) > x
    }
}

fn part1(input: &str) -> Result<()> {
    let count = input.lines()
        .map(Triangle::from_str)
        .filter_map(result::Result::ok)
        .filter(Triangle::is_possible)
        .count();
    println!("Count: {}", count);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let parts = input.split_whitespace().collect::<Vec<_>>();
    let count = parts.chunks(9)
        .flat_map(|parts| {
            vec![Triangle(parts[0].parse().unwrap(),
                          parts[3].parse().unwrap(),
                          parts[6].parse().unwrap()),
                 Triangle(parts[1].parse().unwrap(),
                          parts[4].parse().unwrap(),
                          parts[7].parse().unwrap()),
                 Triangle(parts[2].parse().unwrap(),
                          parts[5].parse().unwrap(),
                          parts[8].parse().unwrap())]
        })
        .filter(Triangle::is_possible)
        .count();
    println!("Count: {}", count);

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
