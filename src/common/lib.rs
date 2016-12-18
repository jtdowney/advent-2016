use errors::*;
use std::env;
use std::str::FromStr;

#[macro_use]
extern crate error_chain;

pub mod errors {
    error_chain!{
        foreign_links {
            Fmt(::std::fmt::Error);
            Io(::std::io::Error);
            ParseInt(::std::num::ParseIntError);
            Utf8(::std::str::Utf8Error);
        }
    }
}

pub enum Problem {
    Part1,
    Part2,
}

impl Problem {
    pub fn from_arg(arg: usize) -> Result<Problem> {
        env::args()
            .nth(arg)
            .ok_or_else(|| "No part specified".into())
            .and_then(|v| v.parse())
    }
}

impl FromStr for Problem {
    type Err = Error;
    fn from_str(part: &str) -> Result<Problem> {
        match part {
            "1" => Ok(Problem::Part1),
            "2" => Ok(Problem::Part2),
            _ => bail!("Unable to parse {}", part),
        }
    }
}
