extern crate common;

use common::get_input;

fn part1(characters: &String) -> usize {
    let mut polymer = Vec::new();
    let mut next: Vec<char> = characters.chars().collect();
    while polymer != next {
        polymer = next.clone();
        next = Vec::new();
        for c in polymer.clone() {
            if let Some(&n) = next.last() {
                if n.to_ascii_lowercase() == c.to_ascii_lowercase() && n != c {
                    next.push(n)
                }
            }
        }
        println!("{} {}", polymer.len(), next.len());
    }
    return polymer.len();
}

fn part2(characters: &String) -> String {
    unimplemented!()
}

fn main() {
    let input = get_input(04, 2018);
    println!("Part 1: {}", part1(&input));
    println!("Part 1: {}", part2(&input));
}
