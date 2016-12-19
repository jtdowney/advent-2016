extern crate common;

use common::Problem;
use common::errors::*;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::str;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
enum Argument {
    Literal(u32),
    Register(char),
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Copy(Argument, char),
    Increment(char),
    Decrement(char),
    JumpNotZero(Argument, i16),
}

impl FromStr for Instruction {
    type Err = Error;
    fn from_str(input: &str) -> Result<Instruction> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        match parts[0] {
            "cpy" => {
                let lhs = parts[1]
                    .parse()
                    .map(Argument::Literal)
                    .unwrap_or_else(|_| {
                        let value = parts[1].chars().nth(0).unwrap();
                        Argument::Register(value)
                    });
                let rhs = parts[2].chars().nth(0).unwrap();
                Ok(Instruction::Copy(lhs, rhs))
            }
            "inc" => Ok(Instruction::Increment(parts[1].chars().nth(0).unwrap())),
            "dec" => Ok(Instruction::Decrement(parts[1].chars().nth(0).unwrap())),
            "jnz" => {
                let lhs = parts[1]
                    .parse()
                    .map(Argument::Literal)
                    .unwrap_or_else(|_| {
                        let value = parts[1].chars().nth(0).unwrap();
                        Argument::Register(value)
                    });
                let rhs = parts[2].parse()?;
                Ok(Instruction::JumpNotZero(lhs, rhs))
            }
            _ => unreachable!(),
        }
    }
}

fn solve(input: &str, registers: &mut HashMap<char, u32>) -> Result<()> {
    let instructions = input.lines()
        .map(Instruction::from_str)
        .collect::<Result<Vec<Instruction>>>()?;

    let mut ip = 0;
    while let Some(instruction) = instructions.get(ip) {
        match *instruction {
            Instruction::Copy(src, dest) => {
                let value = match src {
                    Argument::Literal(v) => v,
                    Argument::Register(n) => registers[&n],
                };

                registers.insert(dest, value);
            }
            Instruction::Increment(reg) => *registers.entry(reg).or_insert(0) += 1,
            Instruction::Decrement(reg) => *registers.entry(reg).or_insert(0) -= 1,
            Instruction::JumpNotZero(src, jump) => {
                let value = match src {
                    Argument::Literal(v) => v,
                    Argument::Register(n) => registers[&n],
                };
                if value != 0 {
                    ip = (jump + ip as i16) as usize;
                    ip -= 1;
                }
            }
        }

        ip += 1;
    }

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut registers = HashMap::new();
    registers.insert('a', 0);
    registers.insert('b', 0);
    registers.insert('c', 0);
    registers.insert('d', 0);

    solve(input, &mut registers)?;

    println!("Register A: {}", registers.get(&'a').unwrap());

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut registers = HashMap::new();
    registers.insert('a', 0);
    registers.insert('b', 0);
    registers.insert('c', 1);
    registers.insert('d', 0);

    solve(input, &mut registers)?;

    println!("Register A: {}", registers.get(&'a').unwrap());

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
