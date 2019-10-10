#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use regex::Regex;
use std::collections::HashMap;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;
#[derive(Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Eq, PartialEq, Debug)]
struct Grid {
    min_point: Point,
    max_point: Point,
    grid_points: Vec<Point>
}

fn main() -> Result<()> {
    let mut input = String::new();
    File::open("input.txt")?.read_to_string(&mut input)?;

    part1(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let anchors: Vec<Point> = input.lines().filter_map(|line| line.parse().ok()).collect();
    let grid = build_grid(&anchors);
    let mut anchor_to_num_points: HashMap<u32, u32> = HashMap::new();
    for grid_point in grid.grid_points.iter() {
        let maybe_closest_anchor = find_closest_anchor(grid_point, &anchors);
        if let Some(closest_anchor) = maybe_closest_anchor {
            let entry = anchor_to_num_points.entry(closest_anchor).or_insert(0);
            *entry += 1;
        }
    }
    println!("{:?}", anchor_to_num_points.values().max().unwrap());
    Ok(())
}

fn find_closest_anchor(grid_point: &Point, anchors: &Vec<Point>) -> Option<u32> {
    let mut shortest_distance = u32::max_value();
    let mut current_anchor = None;

    for i in 0..anchors.len() {
        let anchor = anchors.get(i).unwrap();
        let distance = manhattan_distance(anchor, grid_point);
        if distance == shortest_distance {
            return None;
        } else if distance < shortest_distance {
            current_anchor = Some(i as u32);
            shortest_distance = distance;
        }
    }

    return current_anchor
}

fn manhattan_distance(point1: &Point, point2: &Point) -> u32 {
    return ((point1.x - point2.x).abs() + (point1.y - point2.y).abs()) as u32
}

fn build_grid(anchors: &Vec<Point>) -> Grid {
    let mut min_y = i32::max_value();
    let mut min_x = i32::max_value();
    let mut max_y = 0;
    let mut max_x = 0;

    for anchor in anchors.iter() {
        if anchor.x < min_x {
            min_x = anchor.x
        }
        if anchor.x > max_x {
            max_x = anchor.x
        }
        if anchor.y < min_y {
            min_y = anchor.y
        }
        if anchor.y > max_y {
            max_y = anchor.y
        }
    }

    let mut grid : Vec<Point> = vec![];
    for y in min_y..(max_y+1) {
        for x in min_x..(max_x+1) {
            grid.push(Point{x, y})
        }
    }

    return Grid{
        min_point: Point {x: min_x, y: min_y},
        max_point: Point {x: max_x, y: max_y},
        grid_points: grid
    };
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
