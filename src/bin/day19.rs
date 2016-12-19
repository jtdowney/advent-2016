extern crate common;
extern crate num_integer;

use common::Problem;
use common::errors::*;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Elf {
    id: usize,
    count: usize,
}

fn part1(input: &str) -> Result<()> {
    let size = input.trim().parse()?;
    let mut elves = (0..size)
        .map(|i| {
            Elf {
                id: i + 1,
                count: 1,
            }
        })
        .collect::<Vec<_>>();

    let mut i = 0;
    loop {
        if i >= elves.len() {
            elves.retain(|e| e.count > 0);
            i = 0;
        }

        if elves.len() == 1 {
            break;
        }

        if elves[i].count == 0 {
            i += 1;
            continue;
        }

        let left = num_integer::mod_floor(i + 1, elves.len());
        elves[i].count += elves[left].count;
        elves[left].count = 0;

        i += 1;
    }

    let winner = elves.first().unwrap();
    println!("Winning elf {}", winner.id);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let size = input.trim().parse()?;
    let mut elves = (0..size)
        .map(|i| i + 1)
        .collect::<Vec<_>>();

    let rest = elves.split_off(size / 2);
    let mut left = elves.iter().collect::<VecDeque<_>>();
    let mut right = rest.iter().collect::<VecDeque<_>>();

    while left.len() + right.len() > 1 {
        if left.len() > right.len() {
            left.pop_back();
        } else {
            right.pop_front();
        }

        right.push_back(left.pop_front().unwrap());
        left.push_back(right.pop_front().unwrap());
    }

    let winner = match left.pop_front() {
        Some(v) => v,
        None => right.pop_front().unwrap(),
    };

    println!("Winning elf {}", winner);

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
