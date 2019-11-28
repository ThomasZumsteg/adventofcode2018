extern crate regex;

use regex::Regex;
use std::collections::HashSet;

use common::get_input;
use common::point::Point3d;

type Input = Vec<Bot>;

struct Bot {
    position: Point3d<i32>,
    radius: usize,
}

impl Bot {
    fn from_line(line: &str) -> Bot {
        let re = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
        let cap = re.captures(&line).unwrap();
        Bot {
            position: Point3d::new(
              cap.get(1).unwrap().as_str().parse::<i32>().unwrap(),
              cap.get(2).unwrap().as_str().parse::<i32>().unwrap(),
              cap.get(3).unwrap().as_str().parse::<i32>().unwrap(),
            ),
            radius: cap.get(4).unwrap().as_str().parse::<usize>().unwrap(),
        }
    }
}


fn part1(input: &Input) -> usize {
    let big_bot = input.iter().max_by_key(|b| b.radius).unwrap();
    input.iter().filter(|b| 
        b.position.distance(&big_bot.position) < big_bot.radius
    ).count()
}

fn bot_in_box_range(bots: &Vec<Bot>, position: &Point3d<i32>, radius: usize) -> usize {
    bots.iter()
        .filter(|b| position.distance(&b.position) <= radius + b.radius)
        .count()
}

fn part2(input: &Input) -> usize {
    unimplemented!()
}

fn parse(input: String) -> Input {
    let mut bots = Vec::new();
    for line in input.trim().split('\n') {
        bots.push(Bot::from_line(line));
    }
    bots
}

fn main() {
    let input = parse(get_input(23, 2018));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
