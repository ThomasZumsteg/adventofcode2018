use common::get_input;

use std::collections::{HashMap, HashSet};
use std::fmt;
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

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
struct Cart {
    location: Point,
    heading: Point,
    turns: usize,
}

impl Cart {
    fn turn_left(&mut self) -> Point {
        Point {
            x: self.heading.y,
            y: -self.heading.x,
        }
    }

    fn turn_right(&mut self) -> Point {
        Point {
            x: -self.heading.y,
            y: self.heading.x,
        }
    }
}

#[derive(Clone)]
struct Input {
    track: HashMap<Point, char>,
    carts: HashSet<Cart>
}

impl Input {
    fn step(&mut self) -> HashSet<Point> {
        let positions: HashMap<Point, Vec<&Cart>> = HashMap::new();
        for (i, cart) in self.carts.iter().enumerate() {
            let move_to = &cart.location + &cart.heading;
            if let Some(carts) = positions.get(&cart.location) {
                carts.push(&cart);
            } 
            // positions.get(&move_to).unwrap_or(vec![]).push(&cart);
            cart.location = move_to;

            let track = self.track.get(&cart.location).unwrap();
            // println!("{} {} {:?}", i, track, cart);
            cart.heading = match (track, (cart.heading.x, cart.heading.y)) {
                ('/', (0, -1)) => Point {x:1, y:0},
                ('/', (-1, 0)) => Point {x:0, y:1},
                ('/', (0, 1)) => Point {x:-1, y:0},
                ('/', (1, 0)) => Point {x:0, y:-1},
                ('\\', (0, -1)) => Point {x:-1, y:0},
                ('\\', (-1, 0)) => Point {x:0, y:-1},
                ('\\', (0, 1)) => Point {x:1, y:0},
                ('\\', (1, 0)) => Point {x:0, y:1},
                ('+', _) => {
                    cart.turns += 1;
                    match cart.turns % 3 {
                        0 => cart.turn_right(),
                        1 => cart.turn_left(),
                        2 => cart.heading.clone(),
                        _ => panic!("Never happens")
                    }
                }
                _ => cart.heading.clone(),
            }
        }
        let collisions = HashSet::new();
        for (_, carts) in positions.drain() {
            for cart in carts.drain(..) {
                if let Some(taken) = self.carts.take(cart) {
                    collisions.insert(taken.location);
                }
            }
        }
        collisions
    }
}

fn part1(input: &Input) -> String {
    let mut state: Input = (*input).clone();
    loop {
        let collisions = state.step();
        if let Some(p) = collisions.iter().next() {
            return format!("{},{}", p.x, p.y)
        }
    }
}

fn part2(input: &Input) -> String {
    let mut state: Input = (*input).clone();
    while state.carts.len() > 1 {
        let hits = state.step();
        println!("{}: {:?}", state.carts.len(), hits);
    }
    let p = state.carts.iter().next().unwrap();
    format!("{},{}", p.location.x, p.location.y)
}

fn parse(lines: String) -> Input {
    let mut carts: HashSet<Cart> = HashSet::new();
    let mut track: HashMap<Point, char> = HashMap::new();
    let cart_map = map!{
        '>': ('-', Point { x: 1, y: 0 }),
        '<': ('-', Point { x: -1, y: 0}),
        'v': ('|', Point { x: 0, y: 1}),
        '^': ('|', Point { x: 0, y: -1})
    };
    for (r, line) in lines.split('\n').enumerate() {
        for (c, segment) in line.chars().enumerate() {
            if let Some((segment, heading)) = cart_map.get(&segment) {
                track.insert(Point { x: c as isize, y: r as isize }, *segment);
                carts.insert(Cart { 
                    location: Point { x: c as isize, y: r as isize },
                    heading: heading.clone(),
                    turns: 0,
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
