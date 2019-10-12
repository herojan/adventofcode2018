use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    File::open("input.txt")?.read_to_string(&mut input)?;
    let line = input.lines().next().unwrap();
    let root: Node = line.parse()?;
    part1(&root)?;
    part2(&root)?;
    Ok(())
}

fn part1(root: &Node) -> Result<()> {
    println!("{:?}", root.sum_metadata());
    Ok(())
}

fn part2(root: &Node) -> Result<()> {
    println!("{:?}", root.calculate_value());
    Ok(())
}

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

impl Node {
    fn calculate_value(&self) -> u32 {
        let mut value: u32 = 0;
        if self.children.is_empty() {
            let sum: u32 = self.metadata.iter().sum();
            value += sum;
        } else {
            for m in &self.metadata {
                let child: Option<&Node> = self.children.get((*m-1) as usize);
                value += child.map(|c| c.calculate_value()).unwrap_or(0);
            }
        }

        return value;
    }

    fn sum_metadata(&self) -> u32 {
        let mut sum = self.metadata.iter().sum();
        for child in &self.children {
            sum += child.sum_metadata();
        }
        return sum;
    }
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
