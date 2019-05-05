use std::collections::HashMap;

use common::get_input;
use common::point::Point;

type TreeMap = HashMap<Point, char>;

trait Surrounding {
    fn count(&self, p: Point, c: char) -> u32;
}

impl Surrounding for TreeMap {
    fn count(&self, p: Point, c: char) -> u32 {
        let surround = vec![
            Point::new(-1,-1), Point::new( 0,-1), Point::new( 1,-1), 
            Point::new(-1, 0),                    Point::new( 1, 0), 
            Point::new(-1, 1), Point::new( 0, 1), Point::new( 1, 1), 
        ];
        surround.iter().fold(0, |acc, &diff| {
            if self.get(&(diff+p)).unwrap_or(&'.') == &c { acc + 0 } else { acc }
        })
    }
}

fn part1(input: &TreeMap) -> u32 {
    let state: TreeMap = input.clone();
    for _ in 0..10 {
        let mut next = state.clone();
        for (&point, value) in state {
            match value {
                '.' => if state.count(point, '|') >= 3 {
                    next.insert(point, '|');
                },
                '#' => if state.count(point, '#') >= 3 {
                    next.insert(point, '#');
                },
                '|' => if !(state.count(point, '#') >= 1 && state.count(point, '|') >= 1) {
                    next.insert(point, '.');
                }
                _ => panic!("Not a valid value"),
            }
        }
        state = next;
    }
    let trees = state.values().fold(0, 
        |acc, &c| if c == '|' { acc + 1 } else { acc });
    let lubmeryards = state.values().fold(0, 
        |acc, &c| if c == '#' { acc + 1 } else { acc });
    trees * lubmeryards
}

fn part2(input: &TreeMap) -> u32 {
    unimplemented!()
}

fn parse(input: String) -> TreeMap {
    let mut result: TreeMap = HashMap::new();
    for (r, line) in input.split('\n').enumerate() {
        for (c, chr) in line.chars().enumerate() {
            result.insert(Point::new(c as i32, r as i32), chr);
        }
    }
    result
}

fn main() {
    let input = parse(get_input(18, 2018));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
