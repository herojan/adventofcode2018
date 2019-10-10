#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use regex::Regex;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;
#[derive(Eq, PartialEq, Debug)]
struct Point {
    x: u32,
    y: u32,
}
fn main() -> Result<()> {
    let mut input = String::new();
    File::open("input.txt")?.read_to_string(&mut input)?;

    part1(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let points: Vec<Point> = input.lines().filter_map(|line| line.parse().ok()).collect();
    for point in points.iter() {
        println!("{:?}", point)
    }
    Ok(())
}

impl FromStr for Point {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Point> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\s)*(?P<x>\d+),(\s)*(?P<y>\d+)(\s)*$").unwrap();
        }

        let captures = match RE.captures(s) {
            None => return Err(Box::<dyn Error>::from("invalid point")),
            Some(captures) => captures,
        };

        Ok(Point {
            x: captures["x"].parse()?,
            y: captures["y"].parse()?,
        })
    }
}
