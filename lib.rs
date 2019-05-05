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
        x: i32,
        y: i32,
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
