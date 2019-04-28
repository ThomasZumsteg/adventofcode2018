use std::collections::HashMap;
use std::ops::Add;
use std::fmt;

use regex::Regex;
use common::get_input;

type Input = HashMap<Point, char>;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point(x={}, y={})", self.x, self.y)
    }
}

struct WaterMap<'a> {
    map: &'a Input,
    state: Input,
    front: Vec<Point>,
    lower_right: Point, 
    upper_left: Point, 
}

impl <'a>fmt::Debug for WaterMap<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for y in self.upper_left.y..self.lower_right.y {
            for x in self.upper_left.x..self.lower_right.x {
                result.push(self.get(&Point::new(x, y)));
            }
            result.push('\n');
        }
        write!(f, "{}", result)
    }
}

impl <'a>WaterMap<'a> {
    fn new(map: &'a Input) -> WaterMap<'a> {
        let mut water = HashMap::new();
        water.insert(Point{x:500, y:0}, '+');
        let max_y = map.keys().map(|p| p.y).max().unwrap();
        let min_x = map.keys().map(|p| p.x).min().unwrap();
        let max_x = map.keys().map(|p| p.x).max().unwrap();
        WaterMap {
            map: map,
            state: water,
            upper_left: Point{ x: min_x, y: 0 },
            lower_right: Point{ x: max_x+1, y: max_y+1 },
            front: Vec::new(),
        }
    }

    fn get(&self, point: &Point) -> char {
        if let Some(c) = self.state.get(&point) { *c }
        else if let Some(c) = self.map.get(&point) { *c }
        else { '.' }
    }

    fn step(&mut self) {
        let mut next = Vec::new();
        for p in self.front.clone() {
            let below = p + Point::new(0, 1);
            match self.get(&below) {
                '#' => unimplemented!(),
                '~' => unimplemented!(),
                '|' => unimplemented!(),
                '.' => unimplemented!(),
                err => panic!("Unknow code {:?} as point {:?}", err, p),
            }
        }
        self.front = next;
    }
}


fn part1(input: &Input) -> u32 {
    let map = WaterMap::new(input);
    println!("{:?}", map);
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
            let y = cap[2].parse().unwrap();
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
