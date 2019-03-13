extern crate common;

use common::get_input;
use regex::Regex;

use std::collections::VecDeque;

struct Input {
    n_players: usize,
    n_marbles: usize,
}

trait BoardExt<T> {
    fn rotate(&mut self, steps: i32);
}

impl <T> BoardExt<T> for VecDeque<T> {
    fn rotate(&mut self, steps: i32) {
        for _ in 0..steps.abs() {
            if steps > 0 {
                let moved = self.pop_front().unwrap();
                self.push_back(moved);
            } else {
                let moved = self.pop_back().unwrap();
                self.push_front(moved);
            }
        }
    }
}


fn part1(game: &Input) -> usize {
    let mut board: VecDeque<usize> = VecDeque::with_capacity(game.n_marbles);
    board.push_front(0);
    let mut elves = vec![0; game.n_players];
    for (marble, elf) in (1..game.n_marbles).zip((0..game.n_players).cycle()) {
        if marble % 23 == 0 {
            board.rotate(-7);
            let value = board.pop_front().unwrap();
            elves[elf] += value + marble;
        } else {
            board.rotate(2);
            board.push_front(marble);
        }
    }
    *elves.iter().max().unwrap()
}

fn part2(game: &Input) -> usize {
    part1(&Input {
        n_marbles: game.n_marbles * 100,
        n_players: game.n_players,
    })
}

fn parse(input: String) -> Input {
    let regex = Regex::new(r"(\d+) players; last marble is worth (\d+) points")
        .unwrap();
    let groups = regex.captures(input.trim()).unwrap();
    Input {
        n_players: groups[1].parse().unwrap(), 
        n_marbles: groups[2].parse().unwrap()
    }
}

fn main() {
    let input = parse(get_input(09, 2018));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
