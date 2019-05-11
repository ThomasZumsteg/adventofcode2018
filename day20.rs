use std::collections::{HashSet, HashMap, VecDeque};
use std::iter::FromIterator;

use common::{get_input, map};
use common::point::Point;

type Input = HashMap<Point, HashSet<Point>>;

fn map_distances(map: Input) -> HashMap<Point, usize> {
    let mut queue = VecDeque::from_iter(vec![(Point::new(0, 0), 0)]);
    let mut door_counts: HashMap<Point, usize> = HashMap::new();
    while let Some((position, doors)) = queue.pop_front() {
        if door_counts.contains_key(&position) {
            continue
        }
        door_counts.insert(position, doors);
        for next_position in map.get(&position).unwrap().iter() {
            queue.push_front((next_position.clone(), doors + 1));
        }
    }
    unimplemented!()
}

fn part1(input: &Input) -> usize {
    unimplemented!()
}

fn part2(input: &Input) -> usize {
    unimplemented!()
}

fn parse(text: String) -> Input {
    let mut location = Point::new(0, 0);
    let mut door_map: Input = HashMap::new();
    let mut stack = VecDeque::new();
    let mut queue = VecDeque::from_iter(text.trim().chars());
    assert!(queue.pop_front() == Some('^'));
    assert!(queue.pop_back() == Some('$'));
    while let Some(dir) = queue.pop_front() {
        match dir {
            '(' => { stack.push_front(location); },
            ')' => { location = stack.pop_front().unwrap(); },
            '|' => { location = stack.front().unwrap().clone(); },
            _ => {
                let next_location = location + match dir {
                    'N' => Point::new(0, 1),
                    'S' => Point::new(0, -1),
                    'E' => Point::new(1, 0),
                    'W' => Point::new(-1, 0),
                    _ => panic!("What is this?"),
                };
                door_map.entry(next_location)
                    .or_insert(HashSet::new())
                    .insert(location);
                door_map.entry(location)
                    .or_insert(HashSet::new())
                    .insert(next_location);
                location = next_location;
            },
        }
    }
    assert!(stack.len() == 0);
    door_map
}

fn main() {
    let input = parse(get_input(20, 2018));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
