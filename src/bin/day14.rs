extern crate common;
extern crate crypto;

use common::Problem;
use common::errors::*;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

mod part1 {
    use crypto::digest::Digest;
    use crypto::md5::Md5;
    use std::collections::HashMap;
    pub fn memoized_hash(input: &str, i: u32, hashes: &mut HashMap<u32, String>) -> String {
        hashes.entry(i)
            .or_insert_with(|| {
                let mut digest = Md5::new();
                let input = format!("{}{}", input, i);
                digest.input_str(&input);
                digest.result_str()
            })
            .clone()
    }
}

mod part2 {
    use crypto::digest::Digest;
    use crypto::md5::Md5;
    use std::collections::HashMap;
    pub fn memoized_hash(input: &str, i: u32, hashes: &mut HashMap<u32, String>) -> String {
        hashes.entry(i)
            .or_insert_with(|| {
                let mut digest = Md5::new();
                let mut input = format!("{}{}", input, i);
                for _ in 0..2017 {}
                digest.input_str(&input);
                input = digest.result_str();
                input
            })
            .clone()
    }
}

fn solve<F>(input: &str, hasher: F) -> Result<()>
    where F: Fn(&str, u32, &mut HashMap<u32, String>) -> String
{
    let input = input.trim();
    let mut hashes = HashMap::new();
    let mut keys = Vec::new();
    for i in 0u32.. {
        if keys.len() >= 64 {
            break;
        }

        let digest = hasher(input, i, &mut hashes);
        let candidate = digest.as_bytes()
            .windows(3)
            .find(|triplet| triplet.iter().all(|t| *t == triplet[0]));
        if let Some(c) = candidate {
            for j in 1..1001 {
                let digest = hasher(input, i + j, &mut hashes);
                let found = digest.as_bytes()
                    .windows(5)
                    .any(|quint| quint.iter().all(|q| *q == c[0]));
                if found {
                    keys.push(i);
                }
            }
        }
    }

    let answer = keys.iter().max().unwrap();
    println!("Answer: {}", answer);

    Ok(())
}

fn run() -> Result<()> {
    let problem = Problem::from_arg(1)?;
    let filename = env::args().nth(2).expect("Must provide filename");
    let mut file = File::open(filename)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    match problem {
        Problem::Part1 => solve(&buffer, part1::memoized_hash)?,
        Problem::Part2 => solve(&buffer, part2::memoized_hash)?,
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
