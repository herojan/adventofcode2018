use std::io::Read;
use std::fs::File;
use std::collections::HashMap;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    File::open("input.txt")?.read_to_string(&mut input)?;

    part1(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut twice = 0;
    let mut thrice = 0;
    for line in input.lines() {
        let mut map: HashMap<char, i32> = HashMap::new();
        for c in line.chars() {
            if let Some(x) = map.get_mut(&c) {
                *x = *x + 1;
            } else {
                map.insert(c, 1);
            }
        }
        if map.values().any(|&v| v == 2) {
            twice += 1;
        }
        if map.values().any(|&v| v == 3) {
            thrice += 1;
        }
    }
    println!("{}", twice * thrice);

    Ok(())
}