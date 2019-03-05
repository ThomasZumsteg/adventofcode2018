extern crate common;

use common::get_input;

use std::collections::{HashSet, HashMap};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

struct Areas<T> {
    area: Vec<T>,
    front: Vec<T>
}

impl Point {
    fn distance(&self, q: &Point) -> i32 {
        (self.x - q.x).abs() + (self.y - q.y).abs()
    }

    fn add(&self, other: &Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }

    fn surrounding(&self) -> Vec<Point> {
        vec![
            Point { x: 0, y: 1 },
            Point { x: 0, y: -1 },
            Point { x: 1, y: 0 },
            Point { x: -1, y: 0 }]
            .iter()
            .map(|p| self.add(p))
            .collect()
    }
}

fn part1(points: &Vec<Point>) -> u32 {
    let mut seen: HashSet<&Point> = HashSet::new();
    let mut areas: HashMap<&Point, Areas<&Point>> = HashMap::new();
    let mut max_d: Option<i32> = None;
    for point in points {
        areas.insert(point, Areas { area: Vec::new(), front: vec![point]});
        for q in points {
            if max_d == None || point.distance(q) > max_d.unwrap() {
                max_d = Some(point.distance(q));
            }
        }
    };
    for _ in 0..(max_d.unwrap()/2) {
        for area in areas.values_mut() {
            for point in area.front.to_vec() {
                if seen.contains(point) {
                    continue
                }
                seen.insert(point);
                let closest = areas.keys().fold((Vec::new(), 0), |mut acc, k| {
                    let distance = k.distance(point);
                    if acc.0.len() > 0 && acc.1 > distance {
                        acc.0.clear();
                    } 
                    if acc.0.len() == 0 || acc.1 == distance {
                        acc.0.push(k);
                    }
                    acc
                });
                if closest.0.len() == 1 {
                    area.area.push(closest.0[0]);
                }
            }
        }
    }
    unimplemented!()
}

fn part2(input: &Vec<Point>) -> u32 {
    unimplemented!()
}

fn parse(lines: String) -> Vec<Point> {
    let mut result = Vec::new();
    for line in lines.trim().split('\n') {
        let items: Vec<i32> = line
            .split(", ")
            .map(|v| v.parse::<i32>().unwrap())
            .collect();
        result.push(Point { x:  items[0], y: items[1] });
    }
    return result;
}

fn main() {
    let input = parse(get_input(06, 2018));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
