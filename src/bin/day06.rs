extern crate common;

use common::Problem;
use common::errors::*;
use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::str;

fn solve(input: &str, max_comp: bool) -> Result<()> {
    let word = input.lines()
        .fold(vec![], |mut acc, line| {
            for (i, c) in line.chars().enumerate() {
                if acc.len() <= i {
                    acc.push(vec![c]);
                } else {
                    acc[i].push(c);
                }
            }

            acc
        })
        .into_iter()
        .map(|column| {
            let mut frequencies = BTreeMap::new();
            for c in column {
                *frequencies.entry(c).or_insert(0) += 1;
            }

            let c = if max_comp {
                frequencies.iter().max_by_key(|&(_, freq)| freq).unwrap().0
            } else {
                frequencies.iter().min_by_key(|&(_, freq)| freq).unwrap().0
            };
            *c
        })
        .collect::<String>();
    println!("Secret word is {}", word);

    Ok(())
}

fn run() -> Result<()> {
    let problem = Problem::from_arg(1)?;
    let filename = env::args().nth(2).expect("Must provide filename");
    let mut file = File::open(filename)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    match problem {
        Problem::Part1 => solve(&buffer, true)?,
        Problem::Part2 => solve(&buffer, false)?,
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
