use std::io::prelude::*;
use std::fs::File;

pub fn get_input(day: u8, year: u16) -> String {
    let file_name = format!(".AoC-{:04}-{:02}.tmp", year, day);
    let file = File::open(file_name);
    let mut result = String::new();
    if let Ok(mut f) = file {
        f.read_to_string(&mut result).expect("Unable to read file");
    } else {
        unimplemented!()
    }
    return result;
}

pub mod point {
    use std::ops::{Add, Sub};
    use std::fmt;

    #[derive(Clone, Copy, Eq, PartialEq, Hash)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    impl Point {
        pub fn new(x: i32, y: i32) -> Point {
            Point { x, y }
        }
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point::new(self.x + other.x, self.y + other.y)
        }
    }

    impl Sub for Point {
        type Output = Point;

        fn sub(self, other: Point) -> Point {
            Point::new(self.x - other.x, self.y - other.y)
        }
    }

    impl fmt::Debug for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Point(x={}, y={})", self.x, self.y)
        }
    }
}

pub mod op_code {
    use std::collections::HashMap;

    pub struct Instruction {
        pub name: String,
        pub reg: [usize; 3],
    }

    impl Instruction {
        pub fn new(line: &str) -> Instruction {
            let fields: Vec<&str> = line.split(' ').collect();
            Instruction {
                name: fields[0].to_string(),
                reg: [
                    fields[1].parse::<usize>().unwrap(),
                    fields[2].parse::<usize>().unwrap(),
                    fields[3].parse::<usize>().unwrap(),
                ],
            }
        }
    }

    type Code = [usize; 3];
    type Val = [usize; 6];
    type Op = dyn Fn(Code, Val) -> Val;

    macro_rules! opcode_map(
        { $($key:ident : $value:expr),+ } => {
            {
                let mut m: HashMap<String, &Op> = HashMap::new();
                $(
                    fn $key(code: Code, reg: Val) -> Val {
                        let mut new_val = reg.clone();
                        new_val[code[2]] = $value(code, reg);
                        new_val
                    }
                    m.insert(stringify!($key).to_string(), &$key);
                )+
                m
            }
        };
    );

    pub fn new() -> HashMap<String, &'static Op> {
        opcode_map! {
            addr: |c: Code, r: Val| r[c[0]] + r[c[1]],
            addi: |c: Code, r: Val| r[c[0]] + c[1],
            mulr: |c: Code, r: Val| r[c[0]] * r[c[1]],
            muli: |c: Code, r: Val| r[c[0]] * c[1],
            banr: |c: Code, r: Val| r[c[0]] & r[c[1]],
            bani: |c: Code, r: Val| r[c[0]] & c[1],
            borr: |c: Code, r: Val| r[c[0]] | r[c[1]],
            bori: |c: Code, r: Val| r[c[0]] | c[1],
            setr: |c: Code, r: Val| r[c[0]],
            seti: |c: Code, _: Val| c[0],
            gtir: |c: Code, r: Val| if c[0] > r[c[1]] { 1 } else { 0 },
            gtri: |c: Code, r: Val| if r[c[0]] > c[1] { 1 } else { 0 },
            gtrr: |c: Code, r: Val| if r[c[0]] > r[c[1]] { 1 } else { 0 },
            eqir: |c: Code, r: Val| if c[0] == r[c[1]] { 1 } else { 0 },
            eqri: |c: Code, r: Val| if r[c[0]] == c[1] { 1 } else { 0 },
            eqrr: |c: Code, r: Val| if r[c[0]] == r[c[1]] { 1 } else { 0 }
        }
    }
}
