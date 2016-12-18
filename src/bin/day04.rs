extern crate common;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use common::Problem;
use common::errors::*;
use regex::Regex;
use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::result;
use std::str;
use std::str::FromStr;

lazy_static! {
    static ref ROOM_REGEX: Regex = Regex::new(
        r"^(?P<name>[a-z-]+)-(?P<sector>\d+)\[(?P<checksum>[a-z]{5})\]"
    ).unwrap();
}

#[derive(Debug)]
struct Room {
    name: String,
    checksum: String,
    sector: u32,
}

impl FromStr for Room {
    type Err = Error;
    fn from_str(input: &str) -> Result<Room> {
        let captures = ROOM_REGEX.captures(input).ok_or_else(|| Error::from("no match"))?;
        Ok(Room {
            name: captures.name("name").unwrap().to_string(),
            checksum: captures.name("checksum").unwrap().to_string(),
            sector: captures.name("sector").unwrap().parse().unwrap(),
        })
    }
}

impl Room {
    fn compute_checksum(&self) -> String {
        let mut frequencies = BTreeMap::new();
        for c in self.name.chars() {
            if c == '-' {
                continue;
            }
            *frequencies.entry(c).or_insert(0) += 1;
        }

        let mut frequencies = frequencies.into_iter().collect::<Vec<(char, u8)>>();
        frequencies.sort_by(|a, b| b.1.cmp(&a.1));
        frequencies.into_iter()
            .take(5)
            .map(|(c, _)| c)
            .collect()
    }

    fn is_valid(&self) -> bool {
        self.checksum == self.compute_checksum()
    }

    fn decrypted_name(&self) -> String {
        self.name
            .chars()
            .map(|c| {
                if c == '-' {
                    return ' ';
                }

                let offset = (c as u8) - 97;
                let shifted_offset = ((offset as u32 + self.sector) % 26) as u8;
                (shifted_offset + 97) as char
            })
            .collect()
    }
}

fn part1(input: &str) -> Result<()> {
    let total = input.lines()
        .map(Room::from_str)
        .filter_map(result::Result::ok)
        .filter(Room::is_valid)
        .map(|room| room.sector)
        .sum::<u32>();
    println!("Sector sum: {}", total);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let room = input.lines()
        .map(Room::from_str)
        .filter_map(result::Result::ok)
        .filter(Room::is_valid)
        .find(|room| room.decrypted_name() == "northpole object storage")
        .unwrap();
    println!("Sector ID: {}", room.sector);

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
