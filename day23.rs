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



fn steps(radius: usize) -> Vec<Point3d<i32>> {
    let values = [0i32, radius as i32, -(radius as i32)];
    let mut indexs = [0; 3];
    let mut result: Vec<Point3d<i32>> = Vec::new();
    loop {
        result.push(Point3d::new(
            values[indexs[0]],
            values[indexs[1]],
            values[indexs[2]]
        ));
        for i in 0..3 {
            indexs[i] += 1;
            if indexs[i] < values.len() {
                break
            } else if i+1 < indexs.len() {
                indexs[i] = 0;
            } else {
                return result
            }
        }
    }
}

fn part2(input: &Input) -> usize {
    let position = Point3d::new(0, 0, 0);
    let mut max_radius = 1;
    while bot_in_box_range(&&input, &position, max_radius) < input.len() {
        max_radius *= 2;
    }
    let mut positions = HashSet::new();
    positions.insert(position);
    let mut radius = max_radius;
    while radius > 0 {
        radius /= 2;
        let mut new_positions = HashSet::new();
        let mut seen = HashSet::new();
        let mut best_count = None;
        for &position in positions.iter() {
            for diff in steps(radius) {
                let new_position = diff + position;
                if seen.contains(&new_position) {
                    continue;
                }
                seen.insert(new_position);
                let new_count = bot_in_box_range(input, &new_position, radius);
                if best_count.is_none() || best_count.unwrap() < new_count {
                    new_positions.clear();
                    best_count = Some(new_count);
                }
                if Some(new_count) == best_count {
                    new_positions.insert(new_position);
                }
            }
        }
        positions = new_positions;
    }
    positions.iter().map(|p| p.distance(&Point3d::new(0,0,0))).min().unwrap()
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
