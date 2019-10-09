use std::fs::File;
use std::io::Read;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    File::open("input.txt")?.read_to_string(&mut input)?;
    let line = input.lines().next().unwrap();
    part1(&line)?;
    Ok(())
}

fn part1(line: &str) -> Result<()> {
    let mut bytes = line.as_bytes().to_vec();

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

    println!("{}", String::from_utf8(bytes).unwrap().len());

    Ok(())
}

fn reacts(b1: &u8, b2: &u8) -> bool {
    if b1 > b2 {
        b1 - b2 == 32
    } else {
        b2 - b1 == 32
    }
}
