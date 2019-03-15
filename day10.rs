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

    fn limits(&self) -> (Point2d, Point2d) {
        (Point2d {
            x: self.points.iter().map(|p| p.position.x).min().unwrap(),
            y: self.points.iter().map(|p| p.position.y).min().unwrap(),
        },
        Point2d {
            x: self.points.iter().map(|p| p.position.x).max().unwrap(),
            y: self.points.iter().map(|p| p.position.y).max().unwrap(),
        })
    }

    fn area(&self) -> usize {
        let (min, max) = self.limits();
        ((max.x - min.x) * (max.y - min.y)) as usize
    }

    fn next(&self) -> PointCloud {
        let mut next_step = HashSet::new();
        for item in &self.points {
            next_step.insert(item.step());
        }
        PointCloud { points: next_step }
    }
}


impl ToString for PointCloud {
    fn to_string(&self) -> String {
        let mut output: Vec<Vec<&str>> = Vec::new();
        let (min, max) = self.limits();
        let positions: HashSet<(isize, isize)> = self.points.iter().map(|p| (p.position.x, p.position.y)).collect();
        for r in min.y..max.y+1 {
            let mut row = Vec::new();
            for c in min.x..max.x+1 {
                let this_char = if positions.contains(&(c, r)) { "#" } else { " " };
                row.push(this_char);
            }
            output.push(row);
        }
        return output.iter().map(|row| format!("\n{}", row.join(""))).collect::<String>()
    }
}

fn part1(input: &Input) -> String {
    let points = PointCloud::new(input.iter());
    let mut last = points.clone();
    loop {
        let this = last.next();
        if last.area() < this.area() {
            break
        }
        last = this;
    }
    last.to_string()
}

fn part2(input: &Input) -> usize {
    let points = PointCloud::new(input.iter());
    let mut last = points.clone();
    let mut count = 0;
    loop {
        let this = last.next();
        if last.area() < this.area() {
            return count
        }
        count += 1;
        last = this;
    }
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
