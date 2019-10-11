#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::io::Read;
use std::fs::File;
use std::str::FromStr;
use std::error::Error;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use regex::internal::Char;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    File::open("input.txt")?.read_to_string(&mut input)?;

    part1(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut steps_map: HashMap<char, HashSet<char>> = HashMap::new();
    for line in input.lines() {
        let step: Step = line.parse()?;

        steps_map.entry(step.letter).or_default().insert(step.dep);
        steps_map.entry(step.dep).or_default();
    }

    println!("{}", calculate_step_order(steps_map));

    Ok(())
}

fn calculate_step_order(steps_map: HashMap<char, HashSet<char>>) -> String {
    let mut completed: Vec<char> = vec![];
    let mut maybe_next_step: Option<char> = find_next_step(&steps_map, &completed);

    while let Some(next_step) = maybe_next_step {
        completed.push(next_step);
        maybe_next_step = find_next_step(&steps_map, &completed);
    }
    println!("{:?}", completed);

    return completed.into_iter().collect();
}

fn find_next_step(steps_map: &HashMap<char, HashSet<char>>, completed: &Vec<char>) -> Option<char> {
    let completed: HashSet<char> = completed.iter().map(|&c| c).collect();
    let mut available: Vec<char> = vec![];
    for (&k, deps) in steps_map {
        if deps.is_subset(&completed) {
            available.push(k);
        }
    }
    available.sort();
    available.retain(|l| !completed.contains(l));
    return available.first().map(|c|*c);
}


#[derive(Debug)]
struct Step {
    letter: char,
    dep: char,
}

impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        self.letter == other.letter
    }
}
impl Eq for Step {}

impl FromStr for Step {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Step> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^Step (?P<dep>[A-Z]) must be finished before step (?P<letter>[A-Z]) can begin.$").unwrap();
        }

        let captures = match RE.captures(s) {
            None => return Err(Box::<dyn Error>::from("invalid step")),
            Some(captures) => captures,
        };

        Ok(Step {
            letter: captures["letter"].parse()?,
            dep: captures["dep"].parse()?,
        })
    }
}