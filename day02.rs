extern crate common;

use std::collections::HashMap;

use common::get_input;

fn part1(lines: &Vec<String>) -> u32 {
    let mut threes = 0;
    let mut twos = 0;
    for line in lines {
        let mut counts:HashMap<char, u32> = HashMap::new();
        line.chars().for_each(|c| *counts.entry(c).or_insert(0) += 1);
        if counts.values().any(|&v| v == 3) { threes += 1 }
        if counts.values().any(|&v| v == 2) { twos += 1 }
    }
    return threes * twos;
}

fn part2(lines: &Vec<String>) -> String {
    for (l, head) in lines.into_iter().enumerate() {
        for tail in lines.into_iter().skip(l) {
            let same:String = head
                .chars()
                .into_iter()
                .zip(tail.chars())
                .filter_map(
                    |(a, b)| if a == b { Some(a) } else { None })
                .collect();
            if head.chars().count() - 1 == same.chars().count() {
                return same
            }
        }
    }
    panic!("Cannot find solution");
}

fn parse(lines: String) -> Vec<String> {
    return lines
        .trim()
        .split("\n")
        .into_iter()
        .map(|s| s.to_string())
        .collect();
}

fn main() {
    let input = parse(get_input(02, 2018));
    print!("Part 1: {}\n", part1(&input));
    print!("Part 2: {}\n", part2(&input));
}
