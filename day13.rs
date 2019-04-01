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

impl Point {
    fn distance(&self, other: &Point) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
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

#[derive(Clone, PartialEq, Hash, Eq)]
struct Cart {
    location: Point,
    heading: Point,
    turns: usize,
}

impl fmt::Debug for Cart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut chars = HashMap::new();
        chars.insert(Point { x: -1, y: 0 }, '<');
        chars.insert(Point { x: 1, y: 0 }, '>');
        chars.insert(Point { x: 0, y: -1 }, '^');
        chars.insert(Point { x: 0, y: 1 }, 'v');
        write!(f, "Cart({}, {:?})", chars.get(&self.heading).unwrap(), self.location)
    }
}

impl Cart {
    fn turn_left(&mut self) {
        self.heading = Point {
            x: self.heading.y,
            y: -self.heading.x,
        };
    }

    fn turn_right(&mut self) {
        self.heading = Point {
            x: -self.heading.y,
            y: self.heading.x,
        };
    }

    fn move_forward(&mut self) {
        self.location = &self.location + &self.heading;
    }

    fn turn(&mut self, track: &char) {
        match (track, (self.heading.x, self.heading.y)) {
            ('/', (0, -1)) => self.turn_right(),
            ('/', (-1, 0)) => self.turn_left(),
            ('/', (0, 1)) => self.turn_right(),
            ('/', (1, 0)) => self.turn_left(),
            ('\\', (0, -1)) => self.turn_left(),
            ('\\', (-1, 0)) => self.turn_right(),
            ('\\', (0, 1)) => self.turn_left(),
            ('\\', (1, 0)) => self.turn_right(),
            ('+', _) => {
                self.turns += 1;
                match self.turns % 3 {
                    0 => self.turn_right(),
                    1 => self.turn_left(),
                    2 => (),
                    _ => panic!("Never happens")
                }
            },
            (_, _) => (),
        }
    }

    fn collides_with(&self, carts: &Vec<Cart>) -> Option<Point> {
        for cart in carts {
            if self != cart && self.location.distance(&cart.location) == 0 {
                return Some(self.location.clone())
            }
        }
        None
    }
}

#[derive(Clone)]
struct Input {
    track: HashMap<Point, char>,
    carts: Vec<Cart>
}

impl Input {
    fn step(&mut self) -> HashSet<Point> {
        let mut collisions: HashSet<Point> = HashSet::new();
        let mut next_carts: Vec<Cart> = Vec::new();
        for mut cart in self.carts.clone().drain(..) {
            if let Some(collison) = cart.collides_with(&self.carts) {
                collisions.insert(collison);
                continue;
            }
            cart.move_forward();
            let track = self.track.get(&cart.location).unwrap();
            cart.turn(track);
            if let Some(collison) = cart.collides_with(&self.carts) {
                collisions.insert(collison);
            } else {
                next_carts.push(cart);
            }
        }
        self.carts = next_carts;
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
        if 0 < hits.len() {
            println!("{:?}: {}", hits, state.carts.len());
            println!("{:?}", state.carts);
        }
    }
    let p = state.carts.iter().next().unwrap();
    format!("{},{}", p.location.x, p.location.y)
}

fn parse(lines: String) -> Input {
    let mut carts: Vec<Cart> = Vec::new();
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
                carts.push(Cart { 
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
