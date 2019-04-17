use common::get_input;
use std::collections::{HashMap, VecDeque};
use std::cmp::Ordering;
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::{self, Write};
use std::ops::Add;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i16,
    y: i16,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl Point {
    fn new(x: i16, y: i16) -> Point {
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

type UnitCell = Rc<RefCell<Unit>>;

struct Unit {
    position: Point,
    race: char, 
    hp: i16,
    ap: i16,
}

impl fmt::Debug for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.race {
            'E' => write!(f, "Elf(position: {:?}, hp: {}, ap: {})", 
                          self.position, self.hp, self.ap),
            'G' => write!(f, "Goblin(position: {:?}, hp: {}, ap: {})", 
                          self.position, self.hp, self.ap),
            _ => panic!("Not valid"),
        }
    }
}

impl Unit {
    fn new(position: Point, race: char, hp: i16, ap: i16) -> Unit {
        Unit { position, race, hp, ap }
    }

    fn dead(&self) -> bool {
        self.hp <= 0
    }
}

type Input = HashMap<Point, char>;

enum BoardSpace {
    Wall,
    Space,
    Unit(UnitCell)
}

struct Board {
    units: Vec<UnitCell>,
    map: HashMap<Point, char>,
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        let mut keys: Vec<&Point> = self.map.keys().collect();
        keys.sort();
        let mut curr_row = 0;
        for key in keys {
            if key.y > curr_row {
                curr_row = key.y;
                result.push('\n');
            }
            result.push(match self.get(key) {
                BoardSpace::Unit(u) => u.borrow().race,
                BoardSpace::Space => '.',
                BoardSpace::Wall => '#',
            });
        }
        let mut units = self.units.clone();
        units.sort_by_key(|u| u.borrow().position);
        for unit in units {
            write!(&mut result, "\n{:?}", unit.borrow());
        }
        write!(f, "{}", result)
    }
}

impl Board {
    fn new(input: &HashMap<Point, char>, unit_factory: &Fn(Point, char) -> Option<UnitCell>) -> Board {
        let mut map: HashMap<Point, char> = HashMap::new();
        let mut units: Vec<UnitCell> = Vec::new();
        for (point, chr) in input {
            if let Some(unit) = unit_factory(*point, *chr) {
                units.push(unit);
                map.insert(*point, '.');
            } else {
                map.insert(*point, *chr);
            }
        }
        Board { units, map }
    }

    fn get(&self, position: &Point) -> BoardSpace {
        for unit in self.units.iter() {
            if unit.borrow().position == *position {
                return BoardSpace::Unit(unit.clone());
            }
        }
        if let Some('.') = self.map.get(position) {
            BoardSpace::Space
        } else {
            BoardSpace::Wall
        }
    }

    fn find_path(&self, unit: &UnitCell) -> VecDeque<Point> {
        unimplemented!()
    }

    fn find_enemies(&self, unit: &UnitCell) -> Vec<UnitCell> {
        let all_dirs = [
            Point::new(0, -1),
            Point::new(-1, 0),
            Point::new(1, 0),
            Point::new(0, 1),
        ];
        let mut result = Vec::new();
        for &dir in all_dirs.iter() {
            if let BoardSpace::Unit(other) = self.get(&(unit.borrow().position + dir)) {
                if !other.borrow().dead() && other.borrow().race != unit.borrow().race {
                    result.push(other);
                }
            }
        }
        result.sort_by_key(|u| u.borrow().hp);
        result
    }
}

fn part1(input: &Input) -> u32 {
    fn unit_creator(location: Point, race: char) -> Option<UnitCell> {
        match race {
            'E' => Some(Rc::new(RefCell::new(Unit::new(location, race, 3, 3)))),
            'G' => Some(Rc::new(RefCell::new(Unit::new(location, race, 3, 3)))),
            _ => None 
        }
    }
    let board = Board::new(&input, &unit_creator);
    println!("{:?}", board);
    while board.units.iter().any(|u| u.borrow().race == 'E') && 
        board.units.iter().any(|u| u.borrow().race == 'G')
    {
        let mut order = board.units.clone();
        order.sort_by_key(|u| u.borrow().position);
        for unit in order.iter_mut() {
            if unit.borrow().dead() {
                continue
            }
            let mut path = board.find_path(unit);
            if let Some(pos) = path.pop_front() {
                unit.borrow_mut().position = pos;
            }
            let mut enemies = board.find_enemies(unit);
            enemies.sort_by_key(|e| e.borrow().hp);
            if let Some(enemy) = enemies.pop() {
                enemy.borrow_mut().hp -= unit.borrow().ap;
            }
        }
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
            map.insert(Point::new(c as i16, r as i16), chr);
        }
    }
    map
}

fn main() {
    let input = parse(get_input(15, 2018));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
