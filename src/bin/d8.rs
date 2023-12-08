use std::{collections::HashMap, error::Error, fs::File, io::Read};

use itertools::Itertools;

fn parse_node(node: &str) -> (&str, (&str, &str)) {
    let (from, to) = node.split_once(" = ").unwrap();
    let (to_l, to_r) = to[1..to.len() - 1].split_once(", ").unwrap();
    (from, (to_l, to_r))
}

fn periodicity<'a>(
    mut node: &'a str,
    nodes: &HashMap<&str, (&'a str, &'a str)>,
    directions: &[char],
    end_condition: impl Fn(&str) -> bool,
) -> u64 {
    let mut count = 0;
    for d in directions.iter().cycle() {
        count += 1;
        node = match d {
            'R' => nodes[node].1,
            'L' => nodes[node].0,
            _ => panic!("Invalid char {d}"),
        };
        if end_condition(node) {
            break;
        }
    }
    count
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = String::new();
    File::open("in/8r.txt")?.read_to_string(&mut file)?;
    let mut lines = file.lines();
    let directions = lines.next().unwrap().chars().collect_vec();
    let nodes: HashMap<_, _> = lines.skip(1).map(parse_node).collect();
    let count = periodicity("AAA", &nodes, &directions, |n| n == "ZZZ");
    println!("Total (1): {count}");

    let count = nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|node| periodicity(node, &nodes, &directions, |n| n.ends_with('Z')))
        .fold(1, num::integer::lcm);

    println!("Total (2): {count}");

    Ok(())
}
