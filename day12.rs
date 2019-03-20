extern crate common;

use std::collections::HashMap;
use regex::Regex;

type Pot = char;

struct Pots {
    slots: HashMap<usize, Pot>
}

impl Pots {
    fn new(pots: &mut Iterator<Item=char>, mapping: HashMap<[char; 5], char>) -> Pots {
        let mut slots = HashMap::new();
        for (i, pot) in pots.enumerate() {
            slots.insert(i, pot);
        }
        Pots { slots: slots }
    }
}

fn part1(pots: Pots) -> usize {
    for _ in 0..20 {
        pots = pots.next();
    }
    unimplemented!()
}

fn part2(pots: Pots) -> usize {
    unimplemented!()
}

fn parse(input: String) -> Pots {
    let mut lines = input.trim().split("\n");

    let pots_regex = Regex::new("initial state: ([.#])+").unwrap();
    let pots_line = pots_regex.captures(lines.next().unwrap()).unwrap();

    lines.next();

    let mut mapping: HashMap<[char; 5], Pot> = HashMap::new();
    let mapping_regex = Regex::new("([.#]{5}) => ([#.])").unwrap();
    for line in lines {
        println!("{:?}", line);
        let groups = mapping_regex.captures(line.trim()).unwrap();
        let chars: Vec<char> = groups[1].chars().collect();
        let key: [char; 5] =  [chars[0], chars[1], chars[2], chars[3], chars[4]];
        let value = groups[2].chars().next().unwrap();
        mapping.insert(key, value);
    }
    Pots::new(&mut pots_line[1].chars(), mapping)
}

fn main() {
    let pots = parse(common::get_input(12, 2018));
    println!("Part 1: {}", part1(pots));
    println!("Part 2: {}", part2(pots));
}
