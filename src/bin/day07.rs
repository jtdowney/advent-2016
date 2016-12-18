extern crate common;
extern crate crypto;

use common::Problem;
use common::errors::*;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::Read;
use std::str;

const SEPARATORS: [char; 2] = ['[', ']'];

fn is_abba(part: &[u8]) -> bool {
    let quad = (part[0], part[1], part[2], part[3]);
    match quad {
        (a, b, c, d) if a == d && b == c && a != b => true,
        _ => false,
    }
}

fn to_aba(part: &[u8]) -> (char, char, char) {
    (part[0] as char, part[1] as char, part[2] as char)
}

fn is_aba(triplet: (char, char, char)) -> bool {
    match triplet {
        (a, b, c) if a == c && a != b => true,
        _ => false,
    }
}

fn invert_aba(triplet: (char, char, char)) -> (char, char, char) {
    let (a, b, _) = triplet;
    (b, a, b)
}

fn part1(input: &str) -> Result<()> {
    let count = input.lines()
        .filter(|line| {
            line.split(SEPARATORS.as_ref())
                .enumerate()
                .filter(|&(i, _)| i % 2 == 0)
                .flat_map(|(_, part)| part.as_bytes().windows(4))
                .any(is_abba)
        })
        .filter(|line| {
            !line.split(SEPARATORS.as_ref())
                .enumerate()
                .filter(|&(i, _)| i % 2 != 0)
                .flat_map(|(_, part)| part.as_bytes().windows(4))
                .any(is_abba)
        })
        .count();
    println!("Count: {}", count);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let count = input.lines()
        .filter_map(|line| {
            let found = line.split(SEPARATORS.as_ref())
                .enumerate()
                .filter(|&(i, _)| i % 2 == 0)
                .flat_map(|(_, part)| part.as_bytes().windows(3))
                .map(to_aba)
                .filter(|part| is_aba(*part))
                .map(invert_aba)
                .collect::<HashSet<_>>();
            if found.is_empty() {
                None
            } else {
                Some((line, found))
            }
        })
        .filter(|&(line, ref found)| {
            line.split(SEPARATORS.as_ref())
                .enumerate()
                .filter(|&(i, _)| i % 2 != 0)
                .flat_map(|(_, part)| part.as_bytes().windows(3))
                .map(to_aba)
                .any(|aba| found.contains(&aba))
        })
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
