use std::collections::HashMap;
use std::ops::Add;
use std::fmt;

use regex::Regex;
use common::get_input;

type Input = HashMap<Point, char>;

#[derive(Eq, PartialEq, Hash)]
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

struct WaterMap<'a> {
    _map: &'a Input,
    _state: Input,
    _lower_left: Point, 
    _upper_right: Point, 
}

impl <'a>WaterMap<'a> {
    fn new(map: &'a Input) -> WaterMap<'a> {
        let mut water = HashMap::new();
        water.insert(Point{x:500, y:0}, '+');
        WaterMap {
            _map: map,
            _state: water,
            _lower_left: Point{ x:0, y:0 },
            _upper_right: Point{ x:0, y:0 },
        }
    }
}


fn part1(input: &Input) -> u32 {
    let map = WaterMap::new(input);
    unimplemented!()
}

fn part2(map: &Input) -> u32 {
    unimplemented!()
}

fn parse(lines: String) -> Input {
    let mut result: Input = HashMap::new();
    let regex = Regex::new(r"([xy])=(\d+), ([xy])=(\d+)..(\d+)").unwrap();
    for line in lines.trim().split("\n") {
        let cap = regex.captures(line).unwrap();
        if (&cap[1], &cap[3]) == ("x", "y") {
            let x = cap[2].parse().unwrap();
            for y in cap[4].parse().unwrap()..cap[5].parse().unwrap() {
                result.insert(Point{x, y}, '#');
            }
        } else {
            let y = cap[1].parse().unwrap();
            for x in cap[4].parse().unwrap()..cap[5].parse().unwrap() {
                result.insert(Point{x, y}, '#');
            }
        }
    }
    result
}

fn main() {
    let input = parse(get_input(17, 2018));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
