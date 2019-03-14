extern crate common;

use regex::Regex;
use std::ops::Add;
use std::cmp::{max, min};
use std::collections::HashSet;

type Input = Vec<Point>;

#[derive(Clone)]
struct Point {
    position: Point2d,
    velocity: Point2d,
}

impl Point {
    fn step(&mut self) {
        self.position = &self.position + &self.velocity;
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Point2d {
    x: isize,
    y: isize,
}

impl Add for &Point2d {
    type Output = Point2d;
    fn add(self, other: &Point2d) -> Point2d {
        Point2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn area(upper_right: &Point2d, lower_left: &Point2d) -> usize {
    (upper_right.y - lower_left.y + upper_right.x - lower_left.x) as usize
}


fn part1(input: &Input) -> String {
    let mut points: Input = input.iter().map(|i| i.clone()).collect();
    let mut last_area: Option<(Point2d, Point2d)> = None;
    let mut point_map: HashSet<(isize, isize)> = HashSet::new();
    loop {
        point_map.clear();
        let mut upper_right: Option<Point2d> = None;
        let mut lower_left: Option<Point2d> = None;
        for point in points.iter_mut() {
            point_map.insert((point.position.x, point.position.y));
            point.step();
            if let Some(position) = upper_right.as_mut() {
                *position = Point2d {
                    x: max(position.x, point.position.x),
                    y: max(position.y, point.position.y),
                }
            } else {
                upper_right = Some(point.position.clone());
            }
            if let Some(position) = lower_left.as_mut() {
                *position = Point2d {
                    x: min(position.x, point.position.x),
                    y: min(position.y, point.position.y),
                }
            } else {
                lower_left = Some(point.position.clone());
            }
        }
        
        if let Some(last) = last_area.as_ref() {
            if area(&last.0, &last.1) < area(upper_right.as_ref().unwrap(), lower_left.as_ref().unwrap()) {

                let mut output: Vec<Vec<&str>> = Vec::new();
                for r in last.1.y..last.0.y+1 {
                    let mut row = Vec::new();
                    for c in last.1.x..last.0.x+1 {
                        row.push(if point_map.contains(&(c, r)) { "#" } else { " " });
                    }
                    output.push(row);
                }
                return output.iter().map(|row| format!("\n{}", row.join(""))).collect::<String>()
            }
        }
        last_area = Some((upper_right.unwrap(), lower_left.unwrap()));
    }
    unimplemented!()
}

fn part2(input: &Input) -> String {
    unimplemented!()
}

fn parse(input: String) -> Input {
    let regex = Regex::new(
        r"position=<([ -]\d+), ([ -]\d+)> velocity=<([- ]\d+), ([ -]\d+)>").unwrap();
    input.trim().lines().map(|line| {
        let groups = regex.captures(line.trim()).unwrap();
        Point {
            position: Point2d {
                x: groups[1].trim().parse().unwrap(),
                y: groups[2].trim().parse().unwrap(),
            },
            velocity: Point2d {
                x: groups[3].trim().parse().unwrap(),
                y: groups[4].trim().parse().unwrap(),
            },
        }
    }).collect()
}

fn main() {
    let input = parse(common::get_input(10, 2018));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
