use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    File::open("input.txt")?.read_to_string(&mut input)?;
    let line = input.lines().next().unwrap();
    part1(&line)?;
    Ok(())
}

fn part1(line: &str) -> Result<()> {
    let root: Node = line.parse()?;
    let mut children = root.children;
    let mut all_metadata = root.metadata;
    while !children.is_empty() {
        let child = children.remove(0);
        all_metadata.extend(child.metadata);
        children.extend(child.children);
    }
    let sum: u32 = all_metadata.iter().sum();
    println!("{:?}", sum);
    Ok(())
}

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

impl FromStr for Node {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Node> {
        let nums: Vec<u32> = s.split(' ').filter_map(|c| c.parse::<u32>().ok()).collect();
        let (node, _) = parse_node(&nums, 0);
        Ok(node)
    }
}

fn parse_node(s: &Vec<u32>, start_index: usize) -> (Node, usize) {
    let mut index = start_index;
    let child_count = s[index];
    index += 1;
    let metadata_count = s[index];
    index += 1;
    let children = vec![0; child_count as usize]
        .into_iter()
        .map(|_| {
            let (node, new_index) = parse_node(s, index);
            index = new_index;
            node
        })
        .collect();

    let metadata = vec![0; metadata_count as usize]
        .into_iter()
        .map(|_| {
            let m = *s.get(index).unwrap();
            index += 1;
            m
        })
        .collect();

    return (Node { children, metadata }, index);
}
