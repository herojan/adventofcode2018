use std::io::Read;
use std::fs::File;
use std::collections::HashMap;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    File::open("input.txt")?.read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
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

fn part2(input: &str) -> Result<()> {
    let ids: Vec<&str> = input.lines().collect();

    for i in 0..ids.len() {
        for j in i + 1..ids.len() {
            if let Some(common) = common_characters(&ids[i], &ids[j]) {
                println!("{}", common)
            }
        }
    }
    Ok(())
}

fn common_characters(str1: &str, str2: &str) -> Option<String> {
    if str1.len() != str2.len() {
        return None;
    }

    let mut found_diff = false;

    for (c1, c2) in str1.chars().zip(str2.chars()) {
        if c1 != c2 {
            if found_diff {
                return None;
            }

            found_diff = true;
        }
    }

    if found_diff {
        return Some(
            str1.chars().zip(str2.chars())
                .filter(|&(c1, c2)| c1 == c2)
                .map(|(c, _)| c)
                .collect()
        );
    } else {
        return None;
    }
}