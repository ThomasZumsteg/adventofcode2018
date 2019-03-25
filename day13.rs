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

#[derive(Clone, Debug)]
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
    carts: Vec<Cart>
}

impl Input {
    fn step(&mut self) -> Option<Vec<&Cart>> {
        let mut collisions = Vec::new();
        let mut occupied: HashMap<Point, &Cart> = HashMap::new();
        for (i, cart) in self.carts.iter_mut().enumerate() {
            if occupied.insert(cart.location.clone(), cart).is_none() {
                collisions.push(&*cart);
                continue
            }
            cart.location = &cart.location + &cart.heading;
            if occupied.insert(cart.location.clone(), cart).is_none() {
                collisions.push(&*cart);
                continue
            }
            let track = self.track.get(&cart.location).unwrap();
            println!("{} {} {:?}", i, track, cart);
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
        if collisions.is_empty() {
            None
        } else {
            Some(collisions)
        }
    }
}

fn part1(input: &Input) -> String {
    let mut state: Input = (*input).clone();
    loop {
        if let Some(collisions) = state.step() {
            return format!("{},{}", collisions[0].location.x, collisions[0].location.y)
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
        'v': ('|', Point { x: 0, y: 1}),
        '^': ('|', Point { x: 0, y: -1})
    };
    for (r, line) in lines.trim().split('\n').enumerate() {
        for (c, segment) in line.chars().enumerate() {
            if let Some((segment, heading)) = cart_map.get(&segment) {
                track.insert(Point { x: c as isize, y: r as isize }, *segment);
                carts.push(Cart { 
                    location: Point { x: c as isize, y: r as isize },
                    heading: heading.clone(),
                    turns: 0
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
