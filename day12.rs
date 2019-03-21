extern crate common;

use std::collections::HashMap;
use regex::Regex;

type Pot = char;

#[derive(Clone)]
struct Pots {
    mapping: HashMap<String, char>,
    slots: HashMap<isize, Pot>
}

impl Pots {
    fn new(pots: &mut Iterator<Item=char>, mapping: HashMap<String, char>) -> Pots {
        let mut slots = HashMap::new();
        for (i, pot) in pots.enumerate() {
            slots.insert(i as isize, pot);
        }
        Pots { slots: slots, mapping: mapping }
    }

    fn min(&self) -> isize {
        *self.slots.keys().filter(|i| {
            match self.slots.get(&i) {
                Some('#') => true,
                _ => false
            }
        }).min().unwrap()
    }

    fn max(&self) -> isize {
        *self.slots.keys().max().unwrap()
    }
}

impl Pots {
    fn next(&mut self) {
        let old_slots = self.slots.clone();
        for i in (self.min()-2)..(self.max()+3) {
            let key: String = (i-2..i+3).map(|j| old_slots.get(&j).unwrap_or(&'.')).collect();
            self.slots.insert(i, *self.mapping.get(&key).unwrap_or(&'.'));
        }
    }
}

impl ToString for Pots {
    fn to_string(&self) -> String {
        (self.min()..self.max()).map(|i| self.slots.get(&i).unwrap_or(&'.')).collect()
    }
}

fn part1(start: &Pots) -> isize {
    let mut pots = start.clone();
    for _ in 0..20 {
        println!("{}", pots.to_string());
        pots.next();
    }
    (pots.min()..pots.max()).filter_map(|i| {
        match pots.slots.get(&i) {
            Some('#') => Some(i),
            _ => None
        }
    }).sum()
}

fn part2(pots: &Pots) -> usize {
    unimplemented!()
}

fn parse(input: String) -> Pots {
    let mut lines = input.trim().split("\n");

    let pots_regex = Regex::new("initial state: ([.#]+)").unwrap();
    let pots_line = pots_regex.captures(lines.next().unwrap()).unwrap();

    lines.next();

    let mut mapping: HashMap<String, Pot> = HashMap::new();
    let mapping_regex = Regex::new("([.#]{5}) => ([#.])").unwrap();
    for line in lines {
        let groups = mapping_regex.captures(line.trim()).unwrap();
        let key: String = groups[1].chars().collect();
        let value = groups[2].chars().next().unwrap();
        mapping.insert(key, value);
    }
    Pots::new(&mut pots_line[1].chars(), mapping)
}

fn main() {
    let pots = parse(common::get_input(12, 2018));
    println!("Part 1: {}", part1(&pots));
    println!("Part 2: {}", part2(&pots));
}
