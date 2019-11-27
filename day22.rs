use std::collections::{HashSet, HashMap, BinaryHeap};
use std::cmp::Ordering;

use common::get_input;
use common::point::{self, Point};

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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Gear {
    NONE = 0,
    TORCH = 1,
    CLIMB = 2,
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    distance: usize,
    time: usize,
    position: Point,
    gear: Gear,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Gear {
    fn get_compliment(self, terrain: usize) -> Gear {
        match 3 - terrain - self as usize {
            0 => Gear::NONE,
            1 => Gear::TORCH,
            2 => Gear::CLIMB,
            n => panic!("Not a gear integer {}", n),
        }
    }
}

fn part2(input: &Input) -> usize {
    let mut mapping = GeologicalMap::new(input.depth, input.target);
    let mut seen: HashSet<(Point, Gear)> = HashSet::new();
    let mut queue = BinaryHeap::new();
    queue.push(State {
        distance: Point::new(0, 0).distance(input.target),
        time: 0,
        position: Point::new(0, 0),
        gear: Gear::TORCH
    });

    while let Some(state) = queue.pop() {
        if state.position == mapping.target && state.gear == Gear::TORCH {
            return state.time
        }
        if state.position.x < 0 || state.position.y < 0 ||
           mapping.index(state.position) % 3 == (state.gear as usize) ||
           seen.contains(&(state.position, state.gear)) {
            continue
        }
        seen.insert((state.position, state.gear));
        let other_gear = state.gear.get_compliment(mapping.index(state.position) % 3);
        queue.push(State {
            distance: state.time + 7 + input.target.distance(state.position),
            time: state.time + 7,
            position: state.position,
            gear: other_gear,
        });

        for directions in point::directions().iter() {
            let new_position = state.position + *directions;
            queue.push(
                State {
                    distance: state.time + 1 + input.target.distance(new_position),
                    time: state.time + 1,
                    position: new_position,
                    gear: state.gear,
                });
        }

    }
    panic!("Cannot be done")
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
