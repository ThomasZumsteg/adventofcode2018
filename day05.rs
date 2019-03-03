extern crate common;

use std::collections::HashSet;

use common::get_input;

fn part1(characters: &String) -> usize {
    let mut polymer = Vec::new();
    let mut next: Vec<char> = characters.chars().collect();
    while polymer != next {
        polymer = next;
        next = Vec::new();
        for &head in &polymer {
            if let Some(&tail) = next.last() {
                if head.to_ascii_uppercase() != tail.to_ascii_uppercase() || head == tail {
                    next.push(head);
                } else {
                    next.pop();
                }
            } else {
                next.push(head);
            }
        }
    }
    return polymer.len();
}

fn part2(characters: &String) -> usize {
    let chars: HashSet<char> = characters
        .chars()
        .map(|c| c.to_ascii_uppercase())
        .collect();
    let mut min_count: Option<usize> = None;
    for r in chars {
        let filtered: String = characters
            .chars()
            .filter(|c| c.to_ascii_uppercase() != r)
            .collect();
        let count = part1(&filtered);
        if min_count == None || min_count.unwrap()  > count {
            min_count = Some(count);
        }
    }
    return min_count.unwrap();
}

fn main() {
    let input = get_input(05, 2018);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
