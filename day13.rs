use common::get_input;

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::cell::RefCell;
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

    fn step(&mut self, track: &char) {
        self.move_forward();
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
            (_, _) => panic!("Also never happens"),
        }
    }
}

#[derive(Clone)]
struct Input {
    track: HashMap<Point, char>,
    carts: Vec<RefCell<Cart>>
}

impl Input {
    fn step(&mut self) -> HashSet<Point> {
        let mut positions: HashMap<Point, Vec<&RefCell<Cart>>> = HashMap::new();
        for cell in self.carts.iter() {
            let mut cart = cell.borrow_mut();
            if let Some(carts) = positions.get_mut(&cart.location) {
                carts.push(cell);
            } else {
                positions.insert(cart.location.clone(), vec![cell]);
            }

            let track = self.track.get(&cart.location).unwrap();
            cart.step(track);

            if let Some(carts) = positions.get_mut(&cart.location) {
                carts.push(cell);
            } else {
                positions.insert(cart.location.clone(), vec![cell]);
            }
        }
        let collisions = Vec::new();
        for (position, carts) in positions.drain() {
            if carts.len() > 2 {
                collisions.push(position);
                for cart in carts {
                    self.carts.remove(cart);
                }
            }
        }
        unimplemented!()
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
    let p = state.carts.iter().next().unwrap().borrow();
    format!("{},{}", p.location.x, p.location.y)
}

fn parse(lines: String) -> Input {
    let mut carts: Vec<RefCell<Cart>> = Vec::new();
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
                carts.push(RefCell::new(Cart { 
                    location: Point { x: c as isize, y: r as isize },
                    heading: heading.clone(),
                    turns: 0,
                }));
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
