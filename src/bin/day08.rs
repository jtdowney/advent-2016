extern crate common;

use common::errors::*;
use std::collections::VecDeque;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;
use std::result;
use std::str;

struct Display {
    grid: VecDeque<VecDeque<bool>>,
    height: usize,
}

impl fmt::Debug for Display {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.grid {
            for value in row {
                let c = if *value { '#' } else { '.' };
                write!(formatter, "{}", c)?;
            }
            write!(formatter, "\n")?;
        }
        Ok(())
    }
}

impl Display {
    fn new(width: usize, height: usize) -> Display {
        let mut grid = VecDeque::with_capacity(height);
        for _ in 0..height {
            let mut row = VecDeque::with_capacity(width);
            for _ in 0..width {
                row.push_back(false);
            }
            grid.push_back(row);
        }

        Display {
            grid: grid,
            height: height,
        }
    }

    fn draw_rectangle(&mut self, x: usize, y: usize) {
        for j in 0..y {
            for i in 0..x {
                self.grid[j][i] = true;
            }
        }
    }

    fn rotate_column(&mut self, x: usize, count: usize) {
        for _ in 0..count {
            let value = self.grid[self.height - 1][x];
            for i in (1..self.height).rev() {
                self.grid[i][x] = self.grid[i - 1][x];
            }
            self.grid[0][x] = value;
        }
    }

    fn rotate_row(&mut self, y: usize, count: usize) {
        for _ in 0..count {
            let value = self.grid[y].pop_back().unwrap();
            self.grid[y].push_front(value);
        }
    }

    fn pixel_count(self) -> usize {
        self.grid
            .into_iter()
            .map(|row| row.into_iter().filter(|v| *v).count())
            .sum()
    }

    fn process_line(&mut self, line: &str) -> Result<()> {
        let parts = line.split_whitespace().map(String::from).collect::<Vec<_>>();
        match parts[0].as_str() {
            "rect" => self.process_rect(&parts[1])?,
            "rotate" => self.process_rotate(&parts[1..])?,
            _ => unreachable!(),
        }
        Ok(())
    }

    fn process_rect(&mut self, size: &str) -> Result<()> {
        let parts = size.split('x')
            .map(str::parse)
            .collect::<result::Result<Vec<usize>, ParseIntError>>()?;
        self.draw_rectangle(parts[0], parts[1]);
        Ok(())
    }

    fn process_rotate(&mut self, parts: &[String]) -> Result<()> {
        match parts[0].as_str() {
            "row" => {
                let count = parts[3].parse()?;
                let y = parts[1].split('=')
                    .skip(1)
                    .map(str::parse)
                    .nth(0)
                    .unwrap()?;
                self.rotate_row(y, count);
            }
            "column" => {
                let count = parts[3].parse()?;
                let x = parts[1].split('=')
                    .skip(1)
                    .map(str::parse)
                    .nth(0)
                    .unwrap()?;
                self.rotate_column(x, count);
            }
            _ => unreachable!(),
        };
        Ok(())
    }
}

fn solve(input: &str) -> Result<()> {
    let mut display = Display::new(50, 6);
    for line in input.lines() {
        display.process_line(line)?;
    }

    println!("{:?}", display);
    println!("On count: {}", display.pixel_count());

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
