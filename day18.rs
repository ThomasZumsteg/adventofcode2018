use std::collections::HashMap;
use std::collections::hash_map;

use common::get_input;
use common::point::Point;

#[derive(Clone)]
struct TreeMap {
    map: HashMap<Point, char>,
    upper_left: Point,
    lower_right: Point,
}

impl TreeMap {
    fn new(input: String) -> TreeMap {
        let mut map = HashMap::new();
        let mut max_y = None;
        let mut max_x = None;
        for (r, line) in input.split('\n').enumerate() {
            if max_y == None || max_y.unwrap() < r {
                max_y = Some(r);
            }
            for (c, chr) in line.trim().chars().enumerate() {
                if max_x == None || max_x.unwrap() < r {
                    max_x = Some(c);
                }
                map.insert(Point::new(c as i32, r as i32), chr);
            }
        }
        TreeMap {
            map: map,
            upper_left: Point::new(0, 0),
            lower_right: Point::new(max_x.unwrap() as i32, max_y.unwrap() as i32),
        }
    }

    fn get(&self, p: Point) -> char {
        *self.map.get(&p).unwrap_or(&'.') 
    }

    fn insert(&mut self, key: Point, val: char) -> Option<char> {
        self.map.insert(key, val)
    }

    fn values(&self) -> hash_map::Values<Point, char> {
        self.map.values()
    }
}

trait Surrounding {
    fn adjacent(&self, p: Point, c: char) -> u32;
}

impl Surrounding for TreeMap {
    fn adjacent(&self, p: Point, c: char) -> u32 {
        let surround = vec![
            Point::new(-1,-1), Point::new( 0,-1), Point::new( 1,-1), 
            Point::new(-1, 0),                    Point::new( 1, 0), 
            Point::new(-1, 1), Point::new( 0, 1), Point::new( 1, 1), 
        ];
        surround.iter().fold(0, |acc, &diff| {
            if self.get(diff+p) == c { acc + 1 } else { acc }
        })
    }
}

impl ToString for TreeMap {
    fn to_string(&self) -> String {
        let mut result = String::new();
        for row in self.upper_left.y..self.lower_right.y {
            for col in self.upper_left.x..self.lower_right.x {
                result.push(self.get(Point::new(col, row)));
            }
            result.push('\n');
        }
        result.trim().to_string()
    }
}

impl IntoIterator for &TreeMap {
    type Item = (Point, char);
    type IntoIter = hash_map::IntoIter<Point, char>;
    fn into_iter(self) -> hash_map::IntoIter<Point, char> {
        self.map.clone().into_iter()
    }
}

fn part1(input: &TreeMap) -> u32 {
    let mut state: TreeMap = input.clone();
    for _ in 0..10 {
        let mut next = state.clone();
        for (point, value) in &state {
            match value {
                '.' => if 3 <= state.adjacent(point, '|') {
                    next.insert(point, '|');
                },
                '|' => if 3 <= state.adjacent(point, '#') {
                    next.insert(point, '#');
                },
                '#' => if !(1 <= state.adjacent(point, '#') &&
                            1 <= state.adjacent(point, '|')) {
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
    let n_tests = 1000000000;
    let mut state: TreeMap = input.clone();
    let mut seen: HashMap<String, (usize, TreeMap)> = HashMap::new();
    let mut count = 0;
    while !seen.contains_key(&state.to_string()) && count < n_tests {
        let mut next = state.clone();
        for (point, value) in &state {
            match value {
                '.' => if 3 <= state.adjacent(point, '|') {
                    next.insert(point, '|');
                },
                '|' => if 3 <= state.adjacent(point, '#') {
                    next.insert(point, '#');
                },
                '#' => if !(1 <= state.adjacent(point, '#') &&
                            1 <= state.adjacent(point, '|')) {
                    next.insert(point, '.');
                }
                _ => panic!("Not a valid value"),
            }
        }
        seen.insert(state.to_string(), (count, next.clone()));
        state = next;
        count += 1;
    }
    let mut remaining = 0;
    if let Some((prev, _)) = seen.get(&state.to_string()) {
        remaining = (n_tests - count) % (count - prev);
    }
    for _ in 0..remaining {
        state = seen.get(&state.to_string()).unwrap().1.clone()
    }
    let trees = state.values().fold(0, 
        |acc, &c| if c == '|' { acc + 1 } else { acc });
    let lubmeryards = state.values().fold(0, 
        |acc, &c| if c == '#' { acc + 1 } else { acc });
    trees * lubmeryards
}

fn parse(input: String) -> TreeMap {
    TreeMap::new(input)
}

fn main() {
    let input = parse(get_input(18, 2018));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
