extern crate common;

use common::Problem;
use common::errors::*;
use std::env;
use std::fs::File;
use std::io::Read;
use std::str;

fn parse_number(data: &[u8]) -> Result<usize> {
    let value = str::from_utf8(data)?.parse()?;
    Ok(value)
}

fn read_tuple(data: &[u8]) -> Result<(usize, usize)> {
    let parts = data.split(|b| [b'x', b')'].contains(b))
        .take(2)
        .map(parse_number)
        .collect::<Result<Vec<_>>>()?;
    let length = parts[0];
    let count = parts[1];
    Ok((length, count))
}

fn decompress(input: &[u8]) -> Result<Vec<u8>> {
    let mut result = Vec::with_capacity(input.len());
    let mut i = 0;
    while let Some(c) = input.get(i) {
        match *c {
            b'(' => {
                let next_pos = input[i..].iter().position(|b| *b == b')').unwrap() + i + 1;
                let (length, count) = read_tuple(&input[i + 1..])?;
                let data = &input[next_pos..next_pos + length];
                for _ in 0..count {
                    result.extend_from_slice(data);
                }
                i = next_pos + length;
            }
            c if (c as char).is_whitespace() => i += 1,
            _ => {
                i += 1;
                result.push(*c)
            }
        }
    }

    Ok(result)
}

fn part1(input: &str) -> Result<()> {
    let result = decompress(input.as_bytes())?;
    let result = String::from_utf8(result)?;
    println!("Length: {}", result.len());

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut result = try!(decompress(input.as_bytes()));
    let mut length = result.len();
    loop {
        println!("Current length {}", length);
        result = decompress(&result)?;
        if result.len() == length {
            break;
        } else {
            length = result.len();
        }
    }

    println!("Final length: {}", result.len());

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
