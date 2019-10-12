#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    File::open("input.txt")?.read_to_string(&mut input)?;
    part1(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    for line in input.lines() {
        let game: Game = line.parse()?;
        println!("{:?}", game);
    }
    Ok(())
}

#[derive(Debug)]
struct Game {
    player_count: u32,
    last_marble_points: u32,
}

impl FromStr for Game {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Game> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^(?P<player_count>\d+) players; last marble is worth (?P<last_marble_points>\d+) points$"
            )
            .unwrap();
        }

        let captures = match RE.captures(s) {
            None => return Err(Box::<dyn Error>::from("invalid game")),
            Some(captures) => captures,
        };

        Ok(Game {
            player_count: captures["player_count"].parse()?,
            last_marble_points: captures["last_marble_points"].parse()?,
        })
    }
}
