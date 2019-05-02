use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
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
    front: HashSet<Point>,
    lower_right: Point, 
    upper_left: Point, 
}

impl <'a>fmt::Debug for WaterMap<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for y in self.upper_left.y..self.lower_right.y {
            for x in self.upper_left.x..self.lower_right.x {
                result.push(self.get(Point::new(x, y)));
            }
            result.push('\n');
        }
        write!(f, "{}", result)
    }
}

impl <'a>WaterMap<'a> {
    fn new(map: &'a Input) -> WaterMap<'a> {
        let mut water = HashMap::new();
        water.insert(Point::new(500, 0), '+');
        let max_y = map.keys().map(|p| p.y).max().unwrap();
        let min_x = map.keys().map(|p| p.x).min().unwrap();
        let max_x = map.keys().map(|p| p.x).max().unwrap();
        WaterMap {
            map: map,
            state: water,
            upper_left: Point{ x: min_x, y: 0 },
            lower_right: Point{ x: max_x+1, y: max_y+1 },
            front: HashSet::from_iter(vec![Point::new(500, 1)]),
        }
    }

    fn get(&self, point: Point) -> char {
        if let Some(c) = self.state.get(&point) { *c }
        else if let Some(c) = self.map.get(&point) { *c }
        else { '.' }
    }

    fn set(&mut self, p: Point, ch: char) {
        self.state.insert(p, ch);
    }

    fn in_bounds(&self, point: Point) -> bool {
        self.upper_left.x < point.x && self.upper_left.y < point.y &&
            self.lower_right.x > point.x && self.lower_right.y > point.y
    }

    fn row_is_full(&self, p: Point) -> bool {
        for diff in vec![Point::new(-1, 0), Point::new(1, 0)] {
            let mut q = p;
            while self.get(q) == '|' {
                q = q + diff;
            }
            if self.get(q) != '#' {
                return false;
            }
        }
        true
    }

    fn set_row(&mut self, row: Point, chr: char) {
        self.set(row, '~');
        for diff in vec![Point::new(-1, 0), Point::new(1, 0)] {
            let mut q = row + diff;
            while self.get(q) == '|' {
                self.set(q, chr);
                q = q + diff;
            }
        }
    }

    fn step(&mut self) {
        let mut next = HashSet::new();
        for p in self.front.clone() {
            if self.get(p) != '.' {
                continue
            }
            let surround = (
                self.get(p + Point::new(-1, 0)),
                self.get(p + Point::new(1, 0)),
                self.get(p + Point::new(0, 1)),
            );
            self.set(p, '|');
            match surround {
                ('.', '.', '#') | ('.', '.', '~') => {
                    next.insert(p + Point::new(-1, 0));
                    next.insert(p + Point::new(1, 0));
                },
                ('.', _, '#') | ('.', _, '~') => {
                    next.insert(p + Point::new(-1, 0));
                },
                ( _, '.', '#') | (_, '.', '~') => {
                    next.insert(p + Point::new(1, 0));
                },
                ( _, _, '.') => {
                    next.insert(p + Point::new(0, 1));
                },
                ( '#', _, '#') | ( _, '#', '#') => {
                    if self.row_is_full(p) {
                        self.set_row(p, '~');
                    }
                    next.insert(p + Point::new(-1, 0));
                },
                _ => unimplemented!(),
            }
        }
        next.retain(|p| self.in_bounds(*p));
        self.front = next;
    }
}


fn part1(input: &Input) -> u32 {
    let mut map = WaterMap::new(input);
    while !map.front.is_empty() {
        map.step();
        println!("{:?}\n", map);
    }
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
