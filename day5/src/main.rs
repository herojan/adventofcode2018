use std::fs::File;
use std::io::Read;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    File::open("input.txt")?.read_to_string(&mut input)?;
    let line = input.lines().next().unwrap();
    part1(line)?;
    Ok(())
}

fn part1(line: &str) -> Result<()> {
    let mut shortest_length = u32::max_value();
    for c in 65..91 {
        let mut bytes = line.as_bytes().to_vec();
        let mut i = 0;
        while i < bytes.len() - 1 {
            let b = *bytes.get(i).unwrap() as usize;
            if b == c || b == (c + 32) {
                bytes.remove(i);
            } else {
                i += 1;
            }
        }
        let length = full_reaction(bytes) as u32;
        if length < shortest_length {
            shortest_length = length
        }
    }

    println!("{}", shortest_length);
    Ok(())
}

fn full_reaction(mut bytes: Vec<u8>) -> usize {
    loop {
        let mut reacted = false;
        let mut i = 0;
        while i < bytes.len() - 1 {
            let b1 = bytes.get(i).unwrap();
            let b2 = bytes.get(i + 1).unwrap();

            if reacts(b1, b2) {
                bytes.remove(i);
                bytes.remove(i);
                reacted = true;
            } else {
                i += 1;
            }
        }

        if !reacted {
            break;
        }
    }

    return String::from_utf8(bytes).unwrap().len();
}

fn reacts(b1: &u8, b2: &u8) -> bool {
    if b1 > b2 {
        b1 - b2 == 32
    } else {
        b2 - b1 == 32
    }
}
