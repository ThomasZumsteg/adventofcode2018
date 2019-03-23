use common::get_input;

use std::collections::HashMap;
use std::ops::Add;

macro_rules! map(
    { $($key:tt : $value:expr),+ } => {
        {
            let mut m = HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
    };
);

#[derive(Hash, Eq, PartialEq, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Add for &Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

#[derive(Clone)]
struct Cart {
    location: Point,
    heading: Point,
}

#[derive(Clone)]
struct Input {
    track: HashMap<Point, char>,
    carts: Vec<Cart>
}

impl Input {
    fn step(&mut self) -> Option<Vec<Point>> {
        let mut collections = Vec::new();
        for cart in self.carts.iter_mut() {
            cart.location = &cart.location + &cart.heading;
        }
        if collections.is_empty() {
            None
        } else {
            Some(collections)
        }
    }
}

fn part1(input: &Input) -> String {
    let mut state: Input = (*input).clone();
    loop {
        if let Some(collisions) = state.step() {
            return format!("{},{}", collisions[0].x, collisions[0].y)
        }
    }
}

fn part2(input: &Input) -> String {
    unimplemented!()
}

fn parse(lines: String) -> Input {
    let mut carts: Vec<Cart> = Vec::new();
    let mut track: HashMap<Point, char> = HashMap::new();
    let cart_map = map!{
        '>': ('-', Point { x: 1, y: 0 }),
        '<': ('-', Point { x: -1, y: 0}),
        'v': ('|', Point { x: 0, y: -1}),
        '^': ('|', Point { x: 0, y: 1})
    };
    for (r, line) in lines.trim().split('\n').enumerate() {
        for (c, segment) in line.chars().enumerate() {
            if let Some((segment, heading)) = cart_map.get(&segment) {
                track.insert(Point { x: c as isize, y: r as isize }, *segment);
                carts.push(Cart { 
                    location: Point { x: c as isize, y: r as isize },
                    heading: heading.clone()
                });
            } else if segment != ' '{
                track.insert(Point { x: c as isize, y: r as isize }, segment);
            }
        }
    }
    Input{ track: track, carts: carts }
}

fn main() {
    let input = parse(get_input(13, 2018));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
