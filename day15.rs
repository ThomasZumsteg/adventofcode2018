use common::get_input;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Point {
        Point {x, y}
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        match self.y.cmp(&other.y) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.x.cmp(&other.x),
            Ordering::Greater => Ordering::Greater,
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Unit {
    position: Point,
    race: char, 
    hp: i16,
    ap: i16,
}

impl Unit {
    fn new(position: Point, race: char, hp: i16, ap: i16) -> Unit {
        Unit { position, race, hp, ap }
    }

    fn dead(&self) -> bool {
        self.hp <= 0
    }

    fn find_next_step(&self) -> Option<Point> {
        unimplemented!()
    }
}

type Input = HashMap<Point, char>;

struct Board {
    units: Vec<Rc<RefCell<Unit>>>,
    map: HashMap<Point, Rc<RefCell<Unit>>>,
}

impl Board {
    fn new(input: &HashMap<Point, char>, unit_factory: &Fn(Point, char) -> Rc<RefCell<Unit>>) -> Board {
        let mut map: HashMap<Point, Rc<RefCell<Unit>>> = HashMap::new();
        let mut units: Vec<Rc<RefCell<Unit>>> = Vec::new();
        for (point, chr) in input {
            let unit = unit_factory(*point, *chr);
            units.push(unit.clone());
            map.insert(*point, unit.clone());
        }
        Board { units, map }
    }

    fn take_turn(&mut self) -> Result<(), ()> {
        self.units.sort_by(|a, b| a.position.cmp(&b.position));
        for unit in self.units.iter_mut() {
            if unit.dead() {
                continue
            }
            if let Some(position) = unit.find_next_step() {
                unit.position = position;
            }
            // if let Some(&mut defender) = self.choose_defender(&unit.position) {
            //     defender.hp -= unit.ap;
            // }
        }
        unimplemented!()
    }

    fn choose_defender(&self, position: &Point) -> Option<&mut Unit> {
        unimplemented!()
    }
}

fn part1(input: &Input) -> u32 {
    fn unit_creator(location: Point, race: char) -> Unit {
        match race {
            'E' => Unit::new(location, race, 3, 3),
            'G' => Unit::new(location, race, 3, 3),
            _ => panic!("Unknown race")
        }
    }
    let mut board = Board::new(&input, &unit_creator);
    while true {
        board.take_turn();
    }
    unimplemented!()
}

fn part2(input: &Input) -> u32 {
    unimplemented!()
}

fn parse(text: String) -> Input {
    let mut map: HashMap<Point, char>  = HashMap::new();
    for (r, row) in text.trim().split("\n").enumerate() {
        for (c, chr) in row.chars().enumerate() {
            map.insert(Point::new(c as u32, r as u32), '.');
        }
    }
    map
}

fn main() {
    let input = parse(get_input(15, 2018));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
