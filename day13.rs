use common::get_input;

use std::collections::{HashMap, HashSet};
use std::cell::RefCell;
use std::rc::Rc;
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
}

type CartRef = Rc<RefCell<Cart>>;

#[derive(Clone)]
struct Track {
    track: HashMap<Point, char>,
    carts: Vec<CartRef>
}

impl Track {
    fn step(&mut self) -> HashSet<Point> {
        let mut collisions: HashMap<Point, Vec<CartRef>> = HashMap::new();
        for cart in self.carts.iter_mut() {
            if let Some(carts) = collisions.get_mut(&cart.borrow().location) {
                carts.push(cart.clone());
            } else {
                collisions.insert(
                    cart.borrow().location.clone(),
                    vec![cart.clone()]
                );
            }

            cart.borrow_mut().move_forward();
            let track = self.track.get(&cart.borrow().location).unwrap();
            cart.borrow_mut().turn(track);

            if let Some(carts) = collisions.get_mut(&cart.borrow().location) {
                carts.push(cart.clone());
            } else {
                collisions.insert(
                    cart.borrow().location.clone(),
                    vec![cart.clone()]
                );
            }
        }
        let items: Vec<CartRef> = collisions.values()
            .filter(|values| values.len() > 1)
            .flat_map(|values| values.iter().map(|v| v.clone()))
            .collect();
        println!("{:?}", items);
        for values in collisions.values() {
        }
        unimplemented!();
    }
}

fn part1(track: &HashMap<Point, char>, carts: &Vec<Cart>) -> String {
    let mut state = Track {
        track: track.clone(),
        carts: carts.iter().map(|c| Rc::new(RefCell::new(c.clone()))).collect(),
    };
    loop {
        let collisions = state.step();
        if let Some(p) = collisions.iter().next() {
            return format!("{},{}", p.x, p.y)
        }
    }
}

fn part2(track: &HashMap<Point, char>, carts: &Vec<Cart>) -> String {
    unimplemented!();
    // while state.carts.len() > 1 {
    //     let hits = state.step();
    //     if 0 < hits.len() {
    //         println!("{:?}: {}", hits, state.carts.len());
    //         println!("{:?}", state.carts);
    //     }
    // }
    // let p = state.carts.iter().next().unwrap();
    // format!("{},{}", p.location.x, p.location.y)
}

fn parse(lines: String) -> (HashMap<Point, char>, Vec<Cart>) {
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
    (track, carts)
}

fn main() {
    let (track, carts) = parse(get_input(13, 2018));
    println!("Part 1: {}", part1(&track, &carts));
    println!("Part 2: {}", part2(&track, &carts));
}
