extern crate common;

use common::Problem;
use common::errors::*;
use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::str;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Microchip(u32);
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Bot(u32);

#[derive(Copy, Clone, Debug)]
enum Destination {
    Bot(Bot),
    Output(u32),
}

impl FromStr for Destination {
    type Err = Error;
    fn from_str(input: &str) -> Result<Destination> {
        let parts = input.split_whitespace().collect::<Vec<_>>();
        let value = parts[1].parse()?;
        let destination = match parts[0] {
            "output" => Destination::Output(value),
            "bot" => Destination::Bot(Bot(value)),
            _ => unreachable!(),
        };
        Ok(destination)
    }
}

#[derive(Copy, Clone, Debug)]
enum Command {
    Pickup(Microchip),
    Transfer(Destination, Destination),
}

#[derive(Debug)]
struct Instruction(Bot, Command);

impl Instruction {
    fn parse_pickup(parts: &[&str]) -> Result<Instruction> {
        let value = parts[0].parse()?;
        let bot = parts[4].parse()?;
        Ok(Instruction(Bot(bot), Command::Pickup(Microchip(value))))
    }

    fn parse_transfer(parts: &[&str]) -> Result<Instruction> {
        let bot = parts[0].parse()?;
        let low = parts[4..].join(" ").parse()?;
        let high = parts[9..].join(" ").parse()?;
        Ok(Instruction(Bot(bot), Command::Transfer(low, high)))
    }
}

impl FromStr for Instruction {
    type Err = Error;
    fn from_str(input: &str) -> Result<Instruction> {
        let parts = input.split_whitespace().collect::<Vec<_>>();
        match parts[0] {
            "value" => Instruction::parse_pickup(&parts[1..]),
            "bot" => Instruction::parse_transfer(&parts[1..]),
            _ => unreachable!(),
        }
    }
}

fn solve(input: &str) -> Result<BTreeMap<u32, Microchip>> {
    let mut instructions = input.lines()
        .map(Instruction::from_str)
        .collect::<Result<Vec<_>>>()?;
    let mut bots = BTreeMap::new();
    let mut outputs = BTreeMap::new();

    while !instructions.is_empty() {
        instructions = instructions.into_iter()
            .filter_map(|instruction| {
                let Instruction(bot, command) = instruction;
                let slots_used = {
                    bots.entry(bot).or_insert_with(|| vec![]).len()
                };
                match command {
                    Command::Pickup(microchip) => {
                        let entry = bots.entry(bot).or_insert_with(|| vec![]);
                        entry.push(microchip);
                        None
                    }
                    Command::Transfer(low_dest, high_dest) if slots_used == 2 => {
                        let (low_value, high_value) = {
                            let entry = bots.entry(bot).or_insert_with(|| vec![]);
                            let low_value = {
                                *entry.iter().min().unwrap()
                            };
                            let high_value = {
                                *entry.iter().max().unwrap()
                            };
                            entry.clear();
                            (low_value, high_value)
                        };

                        if low_value == Microchip(17) && high_value == Microchip(61) {
                            println!("{:?} comparing {:?} and {:?}", bot, low_value, high_value);
                        }

                        match low_dest {
                            Destination::Bot(dest_bot) => {
                                let dest_entry = bots.get_mut(&dest_bot).unwrap();
                                dest_entry.push(low_value);
                            }
                            Destination::Output(slot) => {
                                outputs.insert(slot, low_value);
                            }
                        }
                        match high_dest {
                            Destination::Bot(dest_bot) => {
                                let dest_entry = bots.get_mut(&dest_bot).unwrap();
                                dest_entry.push(high_value);
                            }
                            Destination::Output(slot) => {
                                outputs.insert(slot, high_value);
                            }
                        }

                        None
                    }
                    _ => Some(instruction),
                }
            })
            .collect::<Vec<Instruction>>();
    }

    Ok(outputs)
}

fn part1(input: &str) -> Result<()> {
    solve(input)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let outputs = solve(input)?;
    let value = outputs[&0].0 * outputs[&1].0 * outputs[&2].0;
    println!("Result: {}", value);

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
