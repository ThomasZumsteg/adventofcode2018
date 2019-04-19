use common::get_input;
use std::collections::{HashMap, VecDeque, HashSet};
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
        write!(f, "Point(x={}, y={})", self.x, self.y)
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

    fn surrounding(&self) -> Vec<Point> {
        vec![
            *self + Point::new(0, -1),
            *self + Point::new(-1, 0),
            *self + Point::new(1, 0),
            *self + Point::new(0, 1),
        ]
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
            'E' => write!(f, "Elf({:?}, attack={}, hp={})", 
                          self.position, self.ap, self.hp),
            'G' => write!(f, "Goblin({:?}, attack={}, hp={})", 
                          self.position, self.ap, self.hp),
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
    rounds: u32, 
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = format!("Round {}/{}\n", self.rounds, 3);
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
            write!(&mut result, "\n{:?}", unit.borrow())?;
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
        Board { units, map, rounds: 0 }
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
        type State = (Point, VecDeque<Point>);
        let mut seen: HashSet<Point> = HashSet::new();

        let mut queue: Vec<State> = vec![];
        for initial in unit.borrow().position.surrounding() {
            queue.push((initial, VecDeque::new()));
        }
        while !queue.is_empty() {
            let mut new_queue: Vec<State> = Vec::new();
            for (current, path) in queue {
                if seen.contains(&current) {
                    continue;
                }
                seen.insert(current);

                match self.get(&current) {
                    BoardSpace::Wall => continue,
                    BoardSpace::Space => (),
                    BoardSpace::Unit(u) => {
                        if u.borrow().race == unit.borrow().race {
                            continue
                        } else {
                            return path
                        }
                    }
                }
                for diff in current.surrounding() {
                    let mut new_path = path.clone();
                    new_path.push_back(current);
                    new_queue.push((diff, new_path));
                }
            }
            queue = new_queue;
        }
        VecDeque::new()
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
        result.sort_by(|u, v| match u.borrow().hp.cmp(&v.borrow().hp) {
            Ordering::Equal => v.borrow().position.cmp(&u.borrow().position),
            result => result,
        });
        result
    }
}

fn part1(input: &Input) -> u32 {
    fn unit_creator(location: Point, race: char) -> Option<UnitCell> {
        match race {
            'E' => Some(Rc::new(RefCell::new(Unit::new(location, race, 200, 3)))),
            'G' => Some(Rc::new(RefCell::new(Unit::new(location, race, 200, 3)))),
            _ => None 
        }
    }
    let mut board = Board::new(&input, &unit_creator);
    'game: loop {
        let mut order = board.units.clone();
        order.sort_by_key(|u| u.borrow().position);
        for unit in order.iter_mut() {
            if unit.borrow().dead() {
                continue
            }
            if board.units.iter().all(|u|
               u.borrow().dead() || u.borrow().race == unit.borrow().race
            ) {
                board.units.retain(|u| !u.borrow().dead());
                return board.units.iter().fold(0,
                   |acc, u| acc + u.borrow().hp as u32) * board.rounds;
            }
            let mut path = board.find_path(unit);
            if let Some(pos) = path.pop_front() {
                unit.borrow_mut().position = pos;
            }
            let mut enemies = board.find_enemies(unit);
            enemies.sort_by_key(|e| -e.borrow().hp);
            if let Some(enemy) = enemies.pop() {
                enemy.borrow_mut().hp -= unit.borrow().ap;
            }
            board.units.retain(|u| !u.borrow().dead());
        }
        board.rounds += 1;
        // println!("{:?}", board);
    }
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

static SAMPLE_BOARDS: [(&str, u32, u32); 6] = [("#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######", 27730, 4988),
("#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######", 36334, 0),
("#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######", 39514, 31284),
("#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######", 27755, 3478),
("#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######", 28944, 6474),
("#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########", 18740, 1140)];

fn main() {
    for (board, p1_result, p2_result) in SAMPLE_BOARDS.iter() {
        let input = parse(board.to_string());
        assert!(part1(&input)==*p1_result);
        // assert!(*p2_result==0 || part1(&input)==*p2_result);
    }
    let input = parse(get_input(15, 2018));
    println!("Part 1: {}", part1(&input));
    // println!("Part 2: {}", part2(&input));
}
