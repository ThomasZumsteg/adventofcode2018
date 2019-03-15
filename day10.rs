extern crate common;

use regex::Regex;
use std::ops::Add;
use std::collections::HashSet;

type Input = Vec<Point>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    position: Point2d,
    velocity: Point2d,
}

impl Point {
    fn step(&self) -> Point {
         Point {
             position: &self.position + &self.velocity,
             velocity: self.velocity.clone()
         }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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

#[derive(Debug, Clone)]
struct PointCloud {
    points: HashSet<Point>
}

impl PointCloud {
    fn new<'a, I>(items: I) -> PointCloud
        where I: Iterator<Item=&'a Point> {
        PointCloud {
            points: items.map(|p| p.clone()).collect()
        }
    }

    fn max(&self) -> Point2d {
        Point2d {
            x: self.points.iter().map(|p| p.position.x).max().unwrap(),
            y: self.points.iter().map(|p| p.position.y).max().unwrap(),
        }
    }

    fn min(&self) -> Point2d {
        Point2d {
            x: self.points.iter().map(|p| p.position.x).min().unwrap(),
            y: self.points.iter().map(|p| p.position.y).min().unwrap(),
        }
    }

    fn area(&self) -> usize {
        let max = self.max();
        let min = self.min();
        ((max.x - min.x) * (max.y - min.y)) as usize
    }
}


impl ToString for PointCloud {
    fn to_string(&self) -> String {
        let mut output: Vec<Vec<&str>> = Vec::new();
        for r in self.min().y..self.max().y+1 {
            let mut row = Vec::new();
            for c in self.min().x..self.max().x+1 {
                let this_char = if self.points
                    .iter()
                    .any(|p| p.position == Point2d { x: c, y: r }) { "#" } else { " " };
                row.push(this_char);
            }
            output.push(row);
        }
        return output.iter().map(|row| format!("\n{}", row.join(""))).collect::<String>()
    }
}


impl Iterator for &PointCloud {
    type Item = PointCloud;

    fn next(&mut self) -> Option<PointCloud> {
        let mut next_step = HashSet::new();
        for item in &self.points {
            next_step.insert(item.step());
        }
        Some(PointCloud { points: next_step })
    }
}

fn part1(input: &Input) -> String {
    let points = PointCloud::new(input.iter());
    let mut last = points.clone();
    let mut count = 0;
    for this in &points {
        count += 1;
        println!("{}", count);
        if last.area() < this.area() {
            break
        }
        last = this.clone();
    }
    last.to_string()
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
