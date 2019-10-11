#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    File::open("input.txt")?.read_to_string(&mut input)?;
    let steps_map: HashMap<char, HashSet<char>> = build_steps_map(&input)?;
    part1(&steps_map)?;
    part2(&steps_map)?;
    Ok(())
}

fn part1(steps_map: &HashMap<char, HashSet<char>>) -> Result<()> {
    println!("{}", calculate_step_order(steps_map));

    Ok(())
}

fn part2(steps_map: &HashMap<char, HashSet<char>>) -> Result<()> {
    println!("{}", calculate_parallel_step_time(steps_map, 5, 60));

    Ok(())
}

fn calculate_parallel_step_time(
    steps_map: &HashMap<char, HashSet<char>>,
    worker_count: usize,
    work_time: u32,
) -> u32 {
    let mut completed: Vec<char> = vec![];
    let mut workers: Vec<Worker> = vec![0; worker_count]
        .into_iter()
        .map(|_| Worker {
            time_left: 0,
            step: None,
        })
        .collect();

    let mut next_steps: Vec<char> = find_next_steps(&steps_map, &completed);
    let mut count = 0;
    while !next_steps.is_empty() || worker_busy(&workers) {
        take_jobs(&mut next_steps, &mut workers, work_time);
        let mut finished = tick(&mut workers);
        if !finished.is_empty() {
            completed.append(&mut finished);
            next_steps = find_next_steps(&steps_map, &completed);
            take_jobs(&mut next_steps, &mut workers, work_time);
        }
        count += 1;
    }

    return count;
}

fn tick(workers: &mut Vec<Worker>) -> Vec<char> {
    let mut finished_jobs = vec![];
    for worker in workers.iter_mut() {
        if !worker.is_available() {
            if let Some(finished_job) = worker.tick() {
                finished_jobs.push(finished_job)
            }
        }
    }
    finished_jobs.sort();

    return finished_jobs;
}

fn take_jobs(jobs: &mut Vec<char>, workers: &mut Vec<Worker>, work_time: u32) {
    while !jobs.is_empty() && worker_available(workers) {
        let job = jobs.remove(0);
        if workers
            .iter()
            .any(|w| w.step.filter(|&j| j == job).is_some())
        {
            continue;
        }
        for worker in workers.iter_mut() {
            if worker.is_available() {
                worker.take_job(job, work_time);
                break;
            }
        }
    }
}

fn worker_available(workers: &Vec<Worker>) -> bool {
    workers.iter().any(|w| w.is_available())
}

fn worker_busy(workers: &Vec<Worker>) -> bool {
    workers.iter().any(|w| !w.is_available())
}

#[derive(Debug)]
struct Worker {
    time_left: u32,
    step: Option<char>,
}

impl Worker {
    fn is_available(&self) -> bool {
        self.time_left == 0
    }

    fn take_job(&mut self, job: char, work_time: u32) {
        self.time_left += (work_time + (job as u32) - 64);
        self.step = Some(job);
    }

    fn tick(&mut self) -> Option<char> {
        let mut result = None;
        if self.time_left > 0 {
            self.time_left -= 1;
            if self.time_left == 0 {
                result = self.step.clone();
                self.step = None;
            }
        }

        return result;
    }
}

fn build_steps_map(input: &str) -> Result<HashMap<char, HashSet<char>>> {
    let mut steps_map: HashMap<char, HashSet<char>> = HashMap::new();
    for line in input.lines() {
        let step: Step = line.parse()?;

        steps_map.entry(step.letter).or_default().insert(step.dep);
        steps_map.entry(step.dep).or_default();
    }

    return Ok(steps_map);
}

fn calculate_step_order(steps_map: &HashMap<char, HashSet<char>>) -> String {
    let mut completed: Vec<char> = vec![];
    let mut next_steps: Vec<char> = find_next_steps(&steps_map, &completed);

    while !next_steps.is_empty() {
        completed.push(*next_steps.first().unwrap());
        next_steps = find_next_steps(&steps_map, &completed);
    }
    return completed.into_iter().collect();
}

fn find_next_steps(steps_map: &HashMap<char, HashSet<char>>, completed: &Vec<char>) -> Vec<char> {
    let completed: HashSet<char> = completed.iter().map(|&c| c).collect();
    let mut available: Vec<char> = vec![];
    for (&k, deps) in steps_map {
        if deps.is_subset(&completed) {
            available.push(k);
        }
    }
    available.sort();
    available.retain(|l| !completed.contains(l));
    return available;
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
            static ref RE: Regex = Regex::new(
                r"^Step (?P<dep>[A-Z]) must be finished before step (?P<letter>[A-Z]) can begin.$"
            )
            .unwrap();
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
