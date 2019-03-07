extern crate common;

use common::get_input;

use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
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

    fn find_closest<'a>(&self, points: &'a Vec<Point>) -> Option<&'a Point> {
        let mut points_iter = points.into_iter();
        let mut result = Some(points_iter.next().unwrap());
        let mut min_distance = self.distance(&result.unwrap());
        for point in points_iter {
            if self.distance(&point) < min_distance {
                min_distance = self.distance(&point);
                result = Some(point);
            } else if self.distance(&point) == min_distance && result != None {
                result = None;
            }
        }
        return result;
    }
}

fn part1(points: &Vec<Point>) -> u32 {
    let mut enclosed: HashMap<&Point, HashSet<Point>> = HashMap::new();
    let mut boundaries: HashMap<&Point, HashSet<Point>> = HashMap::new();
    let mut max_d: Option<i32> = None;
    for point in points {
        enclosed.insert(point, HashSet::new());
        boundaries.insert(point, HashSet::from_iter(vec![point.clone()]));
        for q in points {
            if max_d == None || point.distance(q) > max_d.unwrap() {
                max_d = Some(point.distance(q));
            }
        }
    };
    let mut seen: HashSet<Point> = HashSet::new();
    for _ in 0..(max_d.unwrap()/2) {
        for boundary in boundaries.values_mut() {
            let mut new_boundary = HashSet::new();
            for point in boundary.iter() {
                if seen.contains(point) {
                    continue;
                }
                seen.insert(point.clone());
                if let Some(closest) = point.find_closest(points) {
                    enclosed.get_mut(closest).unwrap().insert(point.clone());
                }
                for neighbor in point.surrounding() {
                    new_boundary.insert(neighbor);
                }
            }
            *boundary = new_boundary;
        }
    }
    enclosed.iter()
        .filter(|(&k, _)| boundaries.get(k).unwrap().len() == 0)
        .map(|(_, v)| v.len())
        .max()
        .unwrap() as u32
}

fn part2(records: &Vec<Point>) -> usize {
    let limit: i32 = 10000;
    let mut queue = records.clone();
    let mut seen: HashSet<Point> = HashSet::new();
    let mut area: HashSet<Point> = HashSet::new();
    while !queue.is_empty() {
        let point = queue.remove(0);
        if seen.contains(&point) {
            continue;
        }
        seen.insert(point.clone());

        let dist: i32 = records.iter().map(|p| point.distance(p)).sum();
        if limit <= dist {
            continue;
        }
        area.insert(point.clone());

        for neighbor in point.surrounding() {
            queue.push(neighbor);
        }
    }
    return area.len();
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
