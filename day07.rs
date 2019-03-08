extern crate common;

use common::get_input;
use regex::Regex;

use std::collections::{HashMap, HashSet};

type Input = HashMap<String, Vec<String>>;

fn part1(input: &Input) -> u32 {
    let mut roots: HashSet<&String> = input.keys().collect();
    for value in input.values().flat_map(|v| v.iter()) {
        roots.remove(value);
    }
    unimplemented!()
}

fn part2(input: &Input) -> u32 {
    unimplemented!()
}

fn parse(lines: &String) -> Input {
    let mut result = HashMap::new();
    let format = Regex::new("Step (.) must be finished before step (.) can begin.")
        .unwrap();
    for line in lines.trim().split('\n') {
        let groups = format.captures(line).unwrap(); 
        result.entry(groups[2].to_string())
            .or_insert(Vec::new())
            .push(groups[1].to_string());
    }
    return result
}

fn main() {
    let input = parse(&get_input(07, 2018));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
