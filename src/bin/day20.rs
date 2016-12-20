extern crate common;
extern crate interval_tree;

use common::Problem;
use common::errors::*;
use interval_tree::segmentpoint::SegmentPointTree;
use std::env;
use std::fs::File;
use std::io::Read;

fn build_interval_tree(input: &str) -> Result<SegmentPointTree<u32, bool>> {
    let blocked_addresses = input.lines()
        .map(|line| {
            let parts = line.split('-').collect::<Vec<_>>();
            (parts[0].parse(), parts[1].parse())
        })
        .collect::<Vec<_>>();
    let mut firewall = SegmentPointTree::new(u32::min_value(),
                                             u32::max_value(),
                                             false,
                                             Box::new(|a: &bool, b: &bool| *a || *b));
    for (start, end) in blocked_addresses {
        let start = start?;
        let end = end?;
        firewall.insert(start, end, true);
    }

    Ok(firewall)
}

fn part1(input: &str) -> Result<()> {
    let firewall = build_interval_tree(input)?;
    for i in 0..u32::max_value() {
        if let Some(false) = firewall.query(i) {
            println!("{} is allowed", i);
            break;
        }
    }

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let firewall = build_interval_tree(input)?;
    let count = (0..u32::max_value())
        .filter(|i| firewall.query(*i) == Some(false))
        .count();
    println!("{} addresses allowed", count);

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
