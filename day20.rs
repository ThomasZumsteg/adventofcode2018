use std::collections::HashMap;

use common::{get_input, map};
use common::point::Point;

type Input = HashMap<Point, Point>;

fn part1(input: &Input) -> usize {
    unimplemented!()
}

fn part2(input: &Input) -> usize {
    unimplemented!()
}

fn parse(text: String) -> Input {
    let directions = map! {
        "N" => Point::new(0, 1),
        "S" => Point::new(0, -1),
        "E" => Point::new(1, 0),
        "W" => Point::new(-1, 0)
    };
    let location = Point::new(0, 0);
    let mut door_map: Input = HashMap::new();
    unimplemented!()
}

fn main() {
    let input = parse(get_input(20, 2018));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
