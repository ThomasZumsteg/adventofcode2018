extern crate common;

use common::get_input;
use regex::Regex;

use std::collections::{HashMap, HashSet};

type Input = HashMap<String, HashSet<String>>;

fn part1(input: &Input) -> String {
    let mut todo = input.clone();
    let mut result: Vec<String> = Vec::new();
    while !todo.is_empty() {
        let mut to_remove: Vec<String> = Vec::new();
        for (key, values) in todo.clone() {
            if values.iter().all(|v| !todo.contains_key(v)) {
                to_remove.push(key);
            }
        }
        to_remove.sort();
        todo.remove(&to_remove[0]);
        result.push(to_remove[0].clone());
    }
    return result.join("");
}

fn part2(input: &Input) -> u32 {
    let mut todo = input.clone();
    let mut time = 0;
    let mut workers: Vec<(u32, String)> = Vec::new();
    while !todo.is_empty() {
        let t_diff = workers.iter().map(|v| v.0).min().unwrap_or(0) + 1;
        time += t_diff;
        workers = workers.iter().filter_map( |w| {
            if w.0 <= t_diff {
                todo.remove(&w.1);
                None
            } else {
                Some((w.0 - t_diff, w.1.clone()))
            }
        }).collect();
        let mut ready: Vec<String> = Vec::new();
        for key in todo.keys() {
            if todo.values().any(|vals| vals.contains(key)) {
                continue;
            } else if workers.iter()
                .any(|v| &v.1 == key) {
                continue
            }
            ready.push(key.clone());
        }
        ready.sort();
        while workers.len() < 5 && !ready.is_empty() {
            let item = ready.remove(0);
            let first = item.chars().next().unwrap() as u8;
            workers.push(((first - 'A' as u8 + 60) as u32, item.clone()));
        }
    }
    time
}

fn parse(lines: &String) -> Input {
    let mut result = HashMap::new();
    let format = Regex::new("Step (.) must be finished before step (.) can begin.")
        .unwrap();
    for line in lines.trim().split('\n') {
        let groups = format.captures(line).unwrap(); 
        result.entry(groups[1].to_string()).or_insert(HashSet::new());
        result.entry(groups[2].to_string())
            .or_insert(HashSet::new())
            .insert(groups[1].to_string());
    }
    return result
}

fn main() {
    let input = parse(&get_input(07, 2018));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
