extern crate common;

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
    unimplemented!()
}

fn main() {
    let input = common::get_input(11, 2018).trim().parse::<Input>().unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
