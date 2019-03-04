extern crate common;

use common::get_input;

use std::collections::HashSet;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(&self, q: &Point) -> i32 {
        (self.x - q.x).abs() + (self.y - q.y).abs()
    }

    fn add(&self, other: &Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }

    fn surrounding(&self) -> Vec<Point> {
        vec![
            Point { x: 0, y: 1 },
            Point { x: 0, y: -1 },
            Point { x: 1, y: 0 },
            Point { x: -1, y: 0 }]
            .iter()
            .map(|p| self.add(p))
            .collect()
    }
}

fn part1(input: &Vec<Point>) -> u32 {
    let max_d = input.iter()
        .flat_map(|p| input.iter().map(move |q| p.distance(&q)))
        .max()
        .unwrap();
    let mut seen: HashSet<&Point> = HashSet::new();
    let mut queue: &Vec<Point> = input.clone();
    for count in 0..(max_d/2) {
        let new_queue: &Vec<Point> = &Vec::new();
        for p in queue {
            if seen.contains(p) {
                continue;
            }
            seen.insert(p);
            
            for r in input {
            }
        }
    }
    unimplemented!()
}

fn part2(input: &Vec<Point>) -> u32 {
    unimplemented!()
}

fn parse(lines: String) -> Vec<Point> {
    let mut result = Vec::new();
    for line in lines.trim().split('\n') {
        let items: Vec<i32> = line
            .split(", ")
            .map(|v| v.parse::<i32>().unwrap())
            .collect();
        result.push(Point { x:  items[0], y: items[1] });
    }
    return result;
}

fn main() {
    let input = parse(get_input(06, 2018));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
