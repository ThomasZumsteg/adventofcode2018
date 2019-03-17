extern crate common;

use std::cmp::min;

type Input = isize;

fn part1(input: &Input) -> String {
    let mut cells = [[0; 299]; 299];
    for (row, y) in cells.iter_mut().zip(1..) {
        for (cell, x) in row.iter_mut().zip(1..) {
            let rack_id = x + 10;
            *cell = ((((rack_id * y + input) * rack_id) / 100) % 10) - 5;
        }
    }
    let mut max: Option<isize> = None;
    let mut upper_left: Option<String> = None;
    println!("{:?}", cells[0..3].into_iter().map(|row| row[0..3]).collect::Vec<isize>());
    for y in 0..(cells.len() - 2) {
        for x in 0..(cells[y].len() - 2) {
            let this: isize = cells[y..y+3].into_iter()
                .flat_map(|row| row[x..x+3].into_iter())
                .sum();
            if let Some(current) = max {
                if current < this {
                    max = Some(this);
                    upper_left = Some(format!("{},{}", x+1, y+1).to_string());
                }
            } else {
                max = Some(this);
                upper_left = Some(format!("{},{}", x+1, y+1).to_string());
            }
        }
    };
    upper_left.unwrap()
}

fn part2(input: &Input) -> String {
    let mut cells = [[0; 299]; 299];
    for y in 1usize..299 {
        for x in 1usize..299 {
            let rack_id = (x as isize) + 10;
            let mut value = ((((rack_id * (y as isize) + input) * rack_id) / 100) % 10) - 5;
            if y > 1 {
                value += cells[y-2][x-1];
            }
            if x > 1 {
                value += cells[y-1][x-2];
            }
            if x > 1 && y > 1 {
                value -= cells[y-2][x-2];
            }
            cells[y-1][x-1] = value;
        }
    }
    let mut max: Option<isize> = None;
    let mut square: Option<String> = None;
    for y in 1usize..299 {
        for x in 1usize..299 {
            for z in 1usize..min(299-x, 299-y) {
                let mut this = cells[y-1+z][x-1+z];
                this -= cells[y-1][x-1];
                this += cells[y-1+z][x-1];
                this += cells[y-1][x-1+z];
                println!("{},{},{} = {}", x, y, z, this);
                if let Some(current) = max {
                    if current < this {
                        max = Some(this);
                        square = Some(format!("{},{},{}", x, y, z));
                    }
                } else {
                    max = Some(this);
                    square = Some(format!("{},{},{}", x, y, z));
                }
            }
        }
    }
    square.unwrap()
}

fn main() {
    let input = common::get_input(11, 2018).trim().parse::<Input>().unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
