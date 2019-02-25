extern crate common;
extern crate regex;

use std::str;
use std::cmp::{min,max};
use std::collections::HashSet;

use common::get_input;
use regex::Regex;


#[derive(Debug, PartialEq)]
struct Claim {
    id: u32,
    patch: Patch,
}

struct PatchIterator {
    start: Point,
    stop: Point,
    current: Point,
}

impl Iterator for PatchIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        if self.stop.y <= self.current.y {
            return None;
        }
        let result = Some(self.current.clone());
        self.current.x += 1;
        if self.stop.x <= self.current.x {
            self.current.x = self.start.x;
            self.current.y += 1;
        }
        return result;
    }
}

#[derive(Debug, PartialEq)]
struct Patch {
    point: Point,
    size: Point,
}

impl Patch {
    fn overlap(&self, other: &Patch) -> Option<Patch> {
        let min_x = max(self.point.x, other.point.x);
        let min_y = max(self.point.y, other.point.y);
        let max_x = min(self.point.x + self.size.x, other.point.x + other.size.x);
        let max_y = min(self.point.y + self.size.y, other.point.y + other.size.y);
        if min_x < max_x && min_y < max_y {
            return Some(Patch{
                point: Point { x: min_x, y: min_y },
                size: Point { x: max_x - min_x, y: max_y - min_y },
            });
        }
        return None;
    }
}

impl IntoIterator for Patch {
    type Item = Point;
    type IntoIter = PatchIterator;

    fn into_iter(self) -> PatchIterator {
        return PatchIterator {
            current: self.point.clone(),
            start: self.point.clone(),
            stop: Point {
                x: self.point.x + self.size.x,
                y: self.point.y + self.size.y,
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from_str(x: &str, y: &str) -> Point {
        return Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

fn part1(claims: &Vec<Claim>) -> usize {
    let mut overlapping: HashSet<Point> = HashSet::new();
    for (c, claim) in claims.into_iter().enumerate() {
        for other in claims.into_iter().skip(c+1) {
            if let Some(patch) = claim.patch.overlap(&other.patch) {
                for point in patch {
                    overlapping.insert(point);
                }
            }
        }
    }
    return overlapping.into_iter().count();
}

fn part2(claims: &Vec<Claim>) -> u32 {
    for claim in claims {
        let mut has_overlap = false;
        for other in claims {
            if other != claim && claim.patch.overlap(&other.patch).is_some() {
                has_overlap = true;
                break;
            }
        }
        if !has_overlap {
            return claim.id;
        }
    }
    panic!("Unsolveable");
}

fn parse(input: String) -> Vec<Claim> {
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let mut result = Vec::new();
    for line in input.trim().split('\n') {
        let m = re.captures(line).unwrap();
        result.push(Claim { 
            id: m[1].parse().unwrap(),
            patch: Patch {
                point: Point::from_str(&m[2], &m[3]),
                size: Point::from_str(&m[4], &m[5]),
            }
        });
    }
    return result;
}

fn main() {
    let input = parse(get_input(03, 2018));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
