use std::collections::{HashSet, HashMap, VecDeque};

use common::get_input;
use common::point::Point;

struct Input {
    depth: usize,
    target: Point,
}

struct GeologicalMap {
    depth: usize,
    target: Point,
    values: HashMap<Point, usize>
}

impl GeologicalMap {
    fn new(depth: usize, target: Point) -> GeologicalMap {
        GeologicalMap {
            depth: depth,
            target: target,
            values: HashMap::new(),
        }
    }

    fn index(&mut self, point: Point) -> usize {
        if point.x < 0 || point.y < 0 {
            panic!("Point needs to be greater than Point(0, 0)")
        }
        if !self.values.contains_key(&point) {
            let geological_index = if point.x == 0 || point.y == 0 {
                (point.x * 16807 + point.y * 48271) as usize
            } else if point == self.target {
                0
            } else {
                self.index(point+Point::new(-1, 0)) * self.index(point+Point::new(0, -1))
            };
            self.values.insert(point, (geological_index + self.depth) % 20183);
        }
        self.values[&point]
    }
}

fn part1(input: &Input) -> usize {
    let mut mapping = GeologicalMap::new(input.depth, input.target);
    let mut total = 0;
    for y in 0..input.target.y+1 {
        for x in 0..input.target.x+1 {
            total += mapping.index(Point::new(x, y)) % 3;
        }
    }
    total
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Gear {
    NONE = 0,
    TORCH = 1,
    CLIMB = 2,
}

impl Gear {
    fn get_compliment(self, terrain: usize) -> Gear {
        (3 - terrain - self as usize)
    }
}

fn part2(input: &Input) -> usize {
    let mut mapping = GeologicalMap::new(input.depth, input.target);
    let mut seen: HashSet<(Point, Gear)> = HashSet::new();
    let mut queue = VecDeque::new::<>();
    queue.push_back((
        Point::new(0, 0).distance(input.target),
        0,
        Point::new(0, 0),
        Gear::TORCH)
    );

    while !queue.is_empty() {
        let (_, time, position, gear) = queue.pop_front().unwrap();
        if position == mapping.target && gear == Gear::TORCH {
            return time
        }
        if position.x < 0 || position.y < 0 ||
           mapping.index(position) % 3 == (gear as usize) ||
           seen.contains(&(position, gear)) {
            continue
        }
        seen.insert((position, gear));
        let other_gear = gear.get_compliment(mapping.index(position) % 3);

    }
    unimplemented!()
}

fn parse(input: String) -> Input {
    let values: Vec<&str> = input.trim().split('\n')
        .map(|line| line.split(' ').nth(1).unwrap())
        .collect();
    Input {
        depth: values[0].parse::<usize>().unwrap(),
        target: Point::from_str(values[1]),
    }
}

fn main() {
    let input = parse(get_input(22, 2018));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

# Anki
