use std::collections::HashMap;
use std::ops::Add;
use std::fmt;

use regex::Regex;
use common::get_input;

type Input = HashMap<Point, char>;

#[derive(Eq, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Point {
        Point { x, y }
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point(x={}, y={})", self.x, self.y)
    }
}

fn part1(map: &Input) -> u32 {
    unimplemented!()
}

fn part2(map: &Input) -> u32 {
    unimplemented!()
}

fn parse(lines: String) -> Input {
    let regex = Regex::new(r"([xy])=(\d+), ([xy])=(\d+)..(\d+)").unwrap();
    for line in lines.trim().split("\n") {
        println!("{}", line);
    }
    unimplemented!()
}

fn main() {
    let input = parse(get_input(17, 2018));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
