extern crate common;
extern crate num_integer;

use common::errors::*;
use std::env;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
struct Disc {
    position_count: u8,
    current_position: u8,
}

impl FromStr for Disc {
    type Err = Error;
    fn from_str(input: &str) -> Result<Disc> {
        let parts = input.split_whitespace().collect::<Vec<&str>>();
        let part = parts[11];
        let current_position = part[0..part.len() - 1].parse()?;
        Ok(Disc {
            position_count: parts[3].parse()?,
            current_position: current_position,
        })
    }
}

impl Disc {
    fn step(&self, t: u64) -> Disc {
        let next_position =
            num_integer::mod_floor(self.current_position as u64 + t,
                                   self.position_count as u64) as u8;
        Disc {
            position_count: self.position_count,
            current_position: next_position,
        }
    }
}

fn solve(input: &str) -> Result<()> {
    let discs = input.lines()
        .map(Disc::from_str)
        .collect::<Result<Vec<Disc>>>()?;

    for t in 0u64.. {
        for (i, disc) in discs.iter().enumerate() {
            if disc.step(t + i as u64 + 1).current_position != 0 {
                break;
            }

            if i == discs.len() - 1 {
                println!("Fell all the way through at {}", t);
                return Ok(());
            }
        }
    }

    Ok(())
}

fn run() -> Result<()> {
    let filename = env::args().nth(1).expect("Must provide filename");
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
