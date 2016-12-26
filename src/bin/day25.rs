extern crate common;
#[macro_use]
extern crate error_chain;

use common::errors::*;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::str;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
enum Argument {
    Literal(i32),
    Register(char),
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Copy(Argument, Argument),
    Increment(Argument),
    Decrement(Argument),
    JumpNotZero(Argument, Argument),
    Output(Argument),
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
                let rhs = parts[2]
                    .parse()
                    .map(Argument::Literal)
                    .unwrap_or_else(|_| {
                        let value = parts[2].chars().nth(0).unwrap();
                        Argument::Register(value)
                    });
                Ok(Instruction::Copy(lhs, rhs))
            }
            "inc" => {
                Ok(Instruction::Increment(parts[1]
                    .parse()
                    .map(Argument::Literal)
                    .unwrap_or_else(|_| {
                        let value = parts[1].chars().nth(0).unwrap();
                        Argument::Register(value)
                    })))
            }
            "dec" => {
                Ok(Instruction::Decrement(parts[1]
                    .parse()
                    .map(Argument::Literal)
                    .unwrap_or_else(|_| {
                        let value = parts[1].chars().nth(0).unwrap();
                        Argument::Register(value)
                    })))
            }
            "jnz" => {
                let lhs = parts[1]
                    .parse()
                    .map(Argument::Literal)
                    .unwrap_or_else(|_| {
                        let value = parts[1].chars().nth(0).unwrap();
                        Argument::Register(value)
                    });
                let rhs = parts[2]
                    .parse()
                    .map(Argument::Literal)
                    .unwrap_or_else(|_| {
                        let value = parts[2].chars().nth(0).unwrap();
                        Argument::Register(value)
                    });
                Ok(Instruction::JumpNotZero(lhs, rhs))
            }
            "out" => {
                let arg = parts[1]
                    .parse()
                    .map(Argument::Literal)
                    .unwrap_or_else(|_| {
                        let value = parts[1].chars().nth(0).unwrap();
                        Argument::Register(value)
                    });
                Ok(Instruction::Output(arg))
            }
            _ => bail!("unknown instruction during parsing"),
        }
    }
}

fn execute(input: &str, registers: &mut HashMap<char, i32>, count: usize) -> Result<Vec<i32>> {
    let instructions = input.lines()
        .map(Instruction::from_str)
        .collect::<Result<Vec<Instruction>>>()?;

    let mut output = vec![];
    let mut ip = 0;
    while let Some(&instruction) = instructions.get(ip) {
        match instruction {
            Instruction::Copy(src, dest) => {
                let value = match src {
                    Argument::Literal(v) => v,
                    Argument::Register(n) => registers[&n],
                };

                if let Argument::Register(name) = dest {
                    registers.insert(name, value);
                }
            }
            Instruction::Increment(reg) => {
                if let Argument::Register(name) = reg {
                    *registers.entry(name).or_insert(0) += 1;
                }
            }
            Instruction::Decrement(reg) => {
                if let Argument::Register(name) = reg {
                    *registers.entry(name).or_insert(0) -= 1;
                }
            }
            Instruction::JumpNotZero(src, dest) => {
                let value = match src {
                    Argument::Literal(v) => v,
                    Argument::Register(n) => registers[&n],
                };

                let jump = match dest {
                    Argument::Literal(v) => v,
                    Argument::Register(n) => registers[&n],
                };

                if value != 0 {
                    ip = (jump + ip as i32) as usize;
                    ip -= 1;
                }
            }
            Instruction::Output(arg) => {
                let value = match arg {
                    Argument::Literal(v) => v,
                    Argument::Register(n) => registers[&n],
                };

                output.push(value);
                if output.len() == count {
                    return Ok(output);
                }
            }
        }

        ip += 1;
    }

    bail!("Reached end of execution")
}

fn solve(input: &str) -> Result<()> {
    let size = 100;
    let mut registers = HashMap::new();
    let expected: Vec<_> = vec![0, 1]
        .iter()
        .cycle()
        .take(size)
        .cloned()
        .collect();

    for i in 0.. {
        registers.insert('a', i);
        let result = execute(input, &mut registers, size)?;
        if result == expected {
            println!("Answer: {}", i);
            break;
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
