extern crate common;

use std::cmp::min;

type Input = isize;

fn make_cells(size: usize, input: isize) -> Vec<Vec<isize>> {
    let mut cells = vec![vec![0; size]; size];
    for y in 0..size {
        for x in 0..size {
            let rack_id = (x as isize) + 11;
            let mut value = ((((rack_id * (y as isize + 1) + input) * rack_id) / 100) % 10) - 5;
            if y > 0 {
                value += cells[y-1][x];
            }
            if x > 0 {
                value += cells[y][x-1];
            }
            if x > 0 && y > 0 {
                value -= cells[y-1][x-1];
            }
            cells[y][x] = value;
        }
    }
    cells
}

fn value(cells: &Vec<Vec<isize>>, size: (usize, usize, usize)) -> isize {
    let (x, y, z) = size;
    let mut this = cells[y+z+1][x+z+1];
    if y > 0 {
        this -= cells[y][x+z+1];
    }
    if x > 0 {
        this -= cells[y+z+1][x];
    }
    if x > 0 && y > 0 {
        this += cells[y][x];
    }
    this
}

fn part1(input: &Input) -> String {
    let cells = make_cells(300, *input);
    let mut max: Option<isize> = None;
    let mut upper_left: Option<String> = None;
    for y in 0..(300 - 3) {
        for x in 0..(300 - 3) {
            let this = value(&cells, (x, y, 3-1));
            if max == None || max.unwrap() < this {
                max = Some(this);
                upper_left = Some(format!("{},{}", x+2, y+2).to_string());
            }
        }
    };
    upper_left.unwrap()
}

fn part2(input: &Input) -> String {
    let cells = make_cells(300, *input);
    let mut max: Option<isize> = None;
    let mut square: Option<String> = None;
    for y in 0..300 {
        for x in 0..300 {
            for z in 0..min(300-x-1, 300-y-1) {
                let this = value(&cells, (x, y, z));
                if max == None || max.unwrap() < this {
                    max = Some(this);
                    square = Some(format!("{},{},{}", x+2, y+2, z+1));
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
