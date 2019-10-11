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
#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Eq, PartialEq, Debug)]
struct Grid {
    min_point: Point,
    max_point: Point,
    anchors: Vec<Point>,
    grid_points: Vec<Point>,
}

impl Grid {
    fn map_anchors_to_points(&self) -> HashMap<Point, Vec<Point>> {
        let mut anchor_to_num_points: HashMap<Point, Vec<Point>> = HashMap::new();

        for grid_point in &self.grid_points {
            let maybe_anchor = self.find_closest_anchor(grid_point);
            if let Some(closest_anchor) = maybe_anchor {
                let points = anchor_to_num_points.entry(closest_anchor).or_insert(vec![]);
                points.push(*grid_point);
            }
        }

        anchor_to_num_points.retain(|_, v| self.finite_close_points(v));

        return anchor_to_num_points;
    }

    fn finite_close_points(&self, points: &Vec<Point>) -> bool {
        return points.iter().all(|point| {
            !(point.x == self.min_point.x)
                && !(point.x == self.max_point.x)
                && !(point.y == self.min_point.y)
                && !(point.y == self.max_point.y)
        });
    }

    fn find_closest_anchor(&self, grid_point: &Point) -> Option<Point> {
        let (mut closest_anchor, mut more_than_one) = (self.anchors[0], false);

        for &anchor in &self.anchors[1..] {
            let shortest_distance = manhattan_distance(&closest_anchor, &grid_point);
            let distance = manhattan_distance(&anchor, &grid_point);
            if distance == shortest_distance {
                more_than_one = true
            } else if distance < shortest_distance {
                closest_anchor = anchor;
                more_than_one = false
            }
        }
        if more_than_one {
            return None;
        } else {
            return Some(closest_anchor);
        }
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    File::open("input.txt")?.read_to_string(&mut input)?;

    part1(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let anchors: Vec<Point> = input.lines().filter_map(|line| line.parse().ok()).collect();
    let grid = build_grid(anchors);

    let anchor_to_num_points = grid.map_anchors_to_points();
    println!(
        "{:?}",
        anchor_to_num_points
            .iter()
            .map(|(_, v)| v.len())
            .max()
            .unwrap()
    );
    Ok(())
}

fn manhattan_distance(point1: &Point, point2: &Point) -> u32 {
    return ((point1.x - point2.x).abs() + (point1.y - point2.y).abs()) as u32;
}

fn build_grid(anchors: Vec<Point>) -> Grid {
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

    let mut grid: Vec<Point> = vec![];
    for y in min_y..(max_y + 1) {
        for x in min_x..(max_x + 1) {
            grid.push(Point { x, y })
        }
    }

    return Grid {
        min_point: Point { x: min_x, y: min_y },
        max_point: Point { x: max_x, y: max_y },
        anchors,
        grid_points: grid,
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
