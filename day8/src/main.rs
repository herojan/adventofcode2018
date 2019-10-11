use std::io::Read;
use std::fs::File;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    File::open("input.txt")?.read_to_string(&mut input)?;

    part1(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    for line in input.lines() {
        println!("{}", line);
    }

    Ok(())
}