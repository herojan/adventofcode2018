#[macro_use]
extern crate lazy_static;
extern crate chrono;
use chrono::{NaiveDateTime, Timelike};
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use crate::ShiftAction::{StartShift, FallAsleep, WakeUp};
use std::collections::HashMap;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    File::open("input.txt")?.read_to_string(&mut input)?;
    let gid_to_mins_asleep = map_gid_to_mins_asleep(&input)?;

    part1(&gid_to_mins_asleep)?;
    Ok(())
}

fn part1(gid_to_mins_asleep: &HashMap<u32, Vec<u32>>) -> Result<()> {
    let (sleepy_person, mins_asleep) = gid_to_mins_asleep.iter().max_by(|(_, v1), (_, v2)| v1.len().cmp(&v2.len())).unwrap();
    let mut mins_count_map: HashMap<u32, u32> = HashMap::new();
    for &min in mins_asleep.iter() {
        let counter = mins_count_map.entry(min).or_insert(0);
        *counter += 1;
    }
    let (min, _) = mins_count_map.iter().max_by(|(_, v1), (_, v2)| v1.cmp(v2)).unwrap();
    println!("{:?}", min);
    println!("{:?}", sleepy_person);
    println!("{:?}", min * sleepy_person);
    Ok(())
}

fn map_gid_to_mins_asleep(input: &str) -> Result<HashMap<u32, Vec<u32>>> {
    let mut lines: Vec<&str> = input.lines().collect();
    lines.sort();

    let mut gid_to_mins_asleep: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut current_gid: u32 = 0;
    let mut asleep_min: Option<u32> = None;
    for line in lines {
        let shift_change: ShiftChange = line.parse()?;
        if let Some(gid) = shift_change.guard_id {
            current_gid = gid;
        }
        let current_min = shift_change.date_time.minute();
        match shift_change.action {
            FallAsleep => {
                asleep_min = Some(current_min);
            },
            WakeUp => {
                let mins_asleep: Vec<u32> = (asleep_min.unwrap()..(current_min)).collect();
                gid_to_mins_asleep.entry(current_gid).or_insert(vec![]).extend(mins_asleep);
            }
            _ => {
                asleep_min = None;
            },
        }
    }

    return Ok(gid_to_mins_asleep);
}

impl FromStr for ShiftChange {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<ShiftChange> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^\[(?P<dt>\d{4}\-\d{2}\-\d{2} \d{2}:\d{2})\] (?:Guard #(?P<gid>\d+) begins shift)?(?P<asleep>falls asleep)?(?P<awake>wakes up)?$").unwrap();
        }

        let captures = match RE.captures(s) {
            None => return Err(Box::<dyn Error>::from("invalid line")),
            Some(captures) => captures,
        };

        let raw_date_time = &captures["dt"];
        let gid = captures.name("gid").map(|m| m.as_str());
        let action = captures.name("asleep").map(|_| FallAsleep).or(captures.name("awake").map(|_| WakeUp)).unwrap_or(StartShift);

        Ok(ShiftChange{
            guard_id: gid.map(|s| s.parse().unwrap()),
            date_time: NaiveDateTime::parse_from_str(raw_date_time, "%Y-%m-%d %H:%M")?,
            action
        })
    }
}
#[derive(Debug)]
struct ShiftChange {
    guard_id: Option<u32>,
    date_time: NaiveDateTime,
    action: ShiftAction
}
#[derive(Debug)]
enum ShiftAction {
    WakeUp,
    FallAsleep,
    StartShift
}