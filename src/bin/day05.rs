extern crate common;
extern crate crypto;

use common::Problem;
use common::errors::*;
use crypto::digest::Digest;
use crypto::md5::Md5;
use std::env;
use std::fs::File;
use std::io::Read;
use std::str;

fn part1(input: &str) -> Result<()> {
    let mut output = String::new();
    let mut digest = Md5::new();
    for i in 0.. {
        digest.reset();

        let input = format!("{}{}", input.trim(), i);
        digest.input_str(&input);
        let result = digest.result_str();

        if result.starts_with("00000") {
            let c = result.chars().nth(5).unwrap();
            output.push(c);

            if output.len() == 8 {
                break;
            }
        }
    }

    println!("Code: {}", output);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut output = [None; 8];
    let mut digest = Md5::new();
    for i in 0.. {
        digest.reset();

        let input = format!("{}{}", input.trim(), i);
        digest.input_str(&input);
        let result = digest.result_str();

        if result.starts_with("00000") {
            let position = result.chars().nth(5).unwrap();
            if !position.is_digit(10) {
                continue;
            }

            let position = position as usize - 48;
            if position > 7 || output[position].is_some() {
                continue;
            }

            let c = result.chars().nth(6).unwrap();
            output[position] = Some(c);
        }

        if output.iter().all(Option::is_some) {
            break;
        }
    }

    let code = output.iter()
        .map(|c| c.unwrap())
        .collect::<String>();
    println!("Code: {}", code);

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
