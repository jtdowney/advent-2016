extern crate common;
extern crate num_integer;

use common::Problem;
use common::errors::*;
use num_integer::Integer;
use std::env;
use std::fs::File;
use std::io::Read;
use std::str;

fn expand(a: &str) -> String {
    let b = a.chars()
        .rev()
        .map(|c| {
            match c {
                '1' => '0',
                '0' => '1',
                _ => unreachable!(),
            }
        })
        .collect::<String>();

    format!("{}0{}", a, b)
}

fn calculate_checksum(input: &str) -> String {
    input.as_bytes()
        .chunks(2)
        .map(|parts| { if parts[0] == parts[1] { '1' } else { '0' } })
        .collect()
}

fn solve(input: &str, needed_length: usize) -> Result<()> {
    let mut input = input.trim().to_string();
    while input.len() < needed_length {
        input = expand(&input);
    }

    let value = &input[0..needed_length];
    let mut checksum = calculate_checksum(value);
    while checksum.len().is_even() {
        checksum = calculate_checksum(&checksum);
    }

    println!("Checksum: {}", checksum);

    Ok(())
}

fn run() -> Result<()> {
    let problem = Problem::from_arg(1)?;
    let filename = env::args().nth(2).expect("Must provide filename");
    let mut file = File::open(filename)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    match problem {
        Problem::Part1 => solve(&buffer, 272)?,
        Problem::Part2 => solve(&buffer, 35651584)?,
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
