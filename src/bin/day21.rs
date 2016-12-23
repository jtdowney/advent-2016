extern crate common;
#[macro_use]
extern crate error_chain;
extern crate permutohedron;

use common::Problem;
use common::errors::*;
use permutohedron::LexicalPermutation;
use std::env;
use std::fs::File;
use std::io::Read;
use std::str;
use std::str::FromStr;

const START_WORD: &'static str = "abcdefgh";
const TARGET_SCRAMBLE: &'static str = "fbgdceah";

#[derive(Copy, Clone, Debug)]
enum Instruction {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateBased(char),
    ReversePositions(usize, usize),
    MovePosition(usize, usize),
}

impl Instruction {
    fn parse_swap(parts: &[&str]) -> Result<Instruction> {
        match parts[0] {
            "position" => {
                let x = parts[1].parse()?;
                let y = parts[4].parse()?;
                Ok(Instruction::SwapPosition(x, y))
            }
            "letter" => {
                let x = parts[1].chars().nth(0).unwrap();
                let y = parts[4].chars().nth(0).unwrap();
                Ok(Instruction::SwapLetter(x, y))
            }
            _ => bail!("unknown instruction"),
        }
    }

    fn parse_rotate(parts: &[&str]) -> Result<Instruction> {
        match parts[0] {
            "left" => {
                let x = parts[1].parse()?;
                Ok(Instruction::RotateLeft(x))
            }
            "right" => {
                let x = parts[1].parse()?;
                Ok(Instruction::RotateRight(x))
            }
            "based" => {
                let x = parts[5].chars().nth(0).unwrap();
                Ok(Instruction::RotateBased(x))
            }
            _ => bail!("unknown instruction"),
        }
    }

    fn parse_reverse(parts: &[&str]) -> Result<Instruction> {
        let x = parts[1].parse()?;
        let y = parts[3].parse()?;
        Ok(Instruction::ReversePositions(x, y))
    }

    fn parse_move(parts: &[&str]) -> Result<Instruction> {
        let x = parts[1].parse()?;
        let y = parts[4].parse()?;
        Ok(Instruction::MovePosition(x, y))
    }
}

impl FromStr for Instruction {
    type Err = Error;
    fn from_str(input: &str) -> Result<Instruction> {
        let parts: Vec<_> = input.split_whitespace().collect();
        let instruction = match parts[0] {
            "swap" => Instruction::parse_swap(&parts[1..])?,
            "rotate" => Instruction::parse_rotate(&parts[1..])?,
            "reverse" => Instruction::parse_reverse(&parts[1..])?,
            "move" => Instruction::parse_move(&parts[1..])?,
            _ => bail!("unknown instruction"),
        };

        Ok(instruction)
    }
}

fn scramble(start_word: &str, instructions: &[Instruction]) -> String {
    let mut word = start_word.chars().collect::<Vec<_>>();
    for instruction in instructions {
        match *instruction {
            Instruction::SwapPosition(x, y) => word.swap(x, y),
            Instruction::SwapLetter(x, y) => {
                let posx = word.iter().position(|c| *c == x).unwrap();
                let posy = word.iter().position(|c| *c == y).unwrap();
                word.swap(posx, posy);
            }
            Instruction::RotateLeft(x) => {
                for _ in 0..x {
                    let c = word.remove(0);
                    word.push(c);
                }
            }
            Instruction::RotateRight(x) => {
                for _ in 0..x {
                    let c = word.pop().unwrap();
                    word.insert(0, c);
                }
            }
            Instruction::RotateBased(x) => {
                let posx = word.iter().position(|c| *c == x).unwrap();
                let mut count = 1 + posx;
                if posx >= 4 {
                    count += 1;
                }
                for _ in 0..count {
                    let c = word.pop().unwrap();
                    word.insert(0, c);
                }
            }
            Instruction::ReversePositions(x, y) => {
                word = {
                    let (left, mid) = word.split_at_mut(x);
                    let (mid, right) = mid.split_at_mut(y - x + 1);
                    mid.reverse();
                    let mut new_word = vec![];
                    new_word.extend_from_slice(left);
                    new_word.extend_from_slice(mid);
                    new_word.extend_from_slice(right);
                    new_word
                };
            }
            Instruction::MovePosition(x, y) => {
                let c = word.remove(x);
                word.insert(y, c);
            }
        }
    }

    word.into_iter().collect::<String>()
}

fn part1(input: &str) -> Result<()> {
    let instructions = input.lines()
        .map(str::parse)
        .collect::<Result<Vec<Instruction>>>()?;

    let word = scramble(START_WORD, &instructions);
    println!("Word {}", word);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let instructions = input.lines()
        .map(str::parse)
        .collect::<Result<Vec<Instruction>>>()?;

    let mut chars = START_WORD.chars().collect::<Vec<_>>();
    loop {
        let test_word = chars.iter().cloned().collect::<String>();
        if scramble(&test_word, &instructions) == TARGET_SCRAMBLE {
            println!("Word {}", test_word);
            break;
        }

        if !chars.next_permutation() {
            bail!("Reached last possible password");
        }
    }

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
