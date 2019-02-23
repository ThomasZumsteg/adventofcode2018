extern crate common;

use std::collections::HashSet;
use std::vec::Vec;

use common::get_input;

fn part1(input: &Vec<i32>) -> i32 {
    let mut total: i32 = 0;
    for line in input {
        total += line;
    }
    return total;
}

fn part2(input: &Vec<i32>) -> i32 {
    let mut total: i32 = 0;
    let mut seen: HashSet<i32> = HashSet::new();
    for line in input.into_iter().cycle() {
        if seen.contains(&total) {
            break;
        }
        seen.insert(total);
        total += line;
    }
    return total;
}

fn parse(lines: String) -> Vec<i32> {
    let mut result = Vec::new();
    for line in lines.trim().split("\n") {
        result.push(line.parse::<i32>().unwrap());
    }
    return result;
}

fn main() {
    let input = parse(get_input(01, 2018));
    print!("Part 1: {}\n", part1(&input));
    print!("Part 2: {}\n", part2(&input));
}
