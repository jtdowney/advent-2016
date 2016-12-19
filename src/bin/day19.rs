extern crate common;
extern crate num_integer;

use common::Problem;
use common::errors::*;
use std::env;
use std::fs::File;
use std::io::Read;
use std::str;

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
            let before = elves.len();
            elves.retain(|e| e.count > 0);
            i = 0;

            println!("Reached end, removing {} elves", before - elves.len());
        }

        if elves.len() == 1 {
            break;
        }

        if elves[i].count == 0 {
            i += 1;
            continue;
        }

        let left = num_integer::mod_floor(i + 1, elves.len());
        // println!("{} stole {} presents from {}",
        //          elves[i].id,
        //          elves[left].count,
        //          elves[left].id);
        elves[i].count += elves[left].count;
        elves[left].count = 0;

        i += 1;
    }

    println!("{:?}", elves);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
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
        if elves.len() == 1 {
            break;
        }

        if i >= elves.len() {
            println!("Wrapping around, {} elves remaining", elves.len());
            i = 0;
        }

        let count = elves.len();
        let across = num_integer::mod_floor(i + (count / 2), elves.len());
        elves[i].count += elves[across].count;
        elves.remove(across);

        i += 1;
    }

    println!("{:?}", elves);

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
