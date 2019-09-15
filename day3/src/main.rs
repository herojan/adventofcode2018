#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

#[derive(Debug)]
struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

fn main() -> Result<()> {
    let mut input = String::new();
    File::open("input.txt")?.read_to_string(&mut input)?;

    let mut grid = [[0 as i32; 1000]; 1000];
    let claims: Vec<Claim> = input.lines().map(|line| line.parse().unwrap()).collect();

    for claim in claims.iter() {
        let points = claim_to_points(claim);
        for (m, n) in points.iter() {
            grid[*m as usize][*n as usize] += 1;
        }
    }

    part1(&grid)?;
    part2(&claims, &grid)?;
    Ok(())
}

fn part1(grid: &[[i32; 1000]; 1000]) -> Result<()> {
    let sum: usize = grid
        .iter()
        .map(|sub| sub.iter().filter(|&&x| x > 1).count())
        .sum();
    println!("{}", sum);
    Ok(())
}

fn part2(claims: &Vec<Claim>, grid: &[[i32; 1000]; 1000]) -> Result<()> {
    let claim = claims.iter().find(|&claim| {
        let points = claim_to_points(claim);
        return points
            .iter()
            .all(|(x, y)| grid[*x as usize][*y as usize] == 1);
    });
    println!("{}", claim.unwrap().id);
    Ok(())
}

fn claim_to_points(claim: &Claim) -> Vec<(u32, u32)> {
    let mut v = Vec::new();
    for m in claim.x..(claim.x + claim.width) {
        for n in claim.y..(claim.y + claim.height) {
            v.push((m, n));
        }
    }

    return v;
}

impl FromStr for Claim {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Claim> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^#(?P<id>\d+)\s@\s(?P<x>\d+),(?P<y>\d+):\s(?P<w>\d+)x(?P<h>\d+)$")
                    .unwrap();
        }

        let captures = match RE.captures(s) {
            None => return Err(Box::<dyn Error>::from("invalid claim")),
            Some(captures) => captures,
        };

        Ok(Claim {
            id: captures["id"].parse()?,
            x: captures["x"].parse()?,
            y: captures["y"].parse()?,
            width: captures["w"].parse()?,
            height: captures["h"].parse()?,
        })
    }
}
