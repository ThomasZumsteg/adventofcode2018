use common::get_input;
use regex::Regex;

struct Example {
    before: [u32; 4],
    registers: [u32; 4],
    after: [u32; 4],
}

struct Input {
    examples: Vec<Example>,
    program: Vec<[u32; 4]>,
}

fn part1(input: &Input) -> u32 {
    unimplemented!()
}

fn part2(input: &Input) -> u32 {
    unimplemented!()
}

fn parse(input: String) -> Input {
    let mut lines = input.trim().split('\n');
    let mut examples: Vec<Example> = vec![];
    let re_before = Regex::new(r"Before: \[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
    let re_registers = Regex::new(r"(\d+) (\d+) (\d+) (\d+)").unwrap();
    let re_after = Regex::new(r"After:  \[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
    loop {
        let before = lines.next().unwrap();
        if before == "" {
            lines.next();
            break
        }
        let registers = lines.next().unwrap();
        let after = lines.next().unwrap();
        examples.push(Example {
            before: {
                let cap = re_before.captures(before).unwrap();
                [
                    cap[1].parse::<u32>().unwrap(),
                    cap[2].parse::<u32>().unwrap(),
                    cap[3].parse::<u32>().unwrap(),
                    cap[4].parse::<u32>().unwrap(),
                ]
            },
            registers: {
                let cap = re_registers.captures(registers).unwrap();
                [
                    cap[1].parse::<u32>().unwrap(),
                    cap[2].parse::<u32>().unwrap(),
                    cap[3].parse::<u32>().unwrap(),
                    cap[4].parse::<u32>().unwrap(),
                ]
            },
            after: {
                let cap = re_after.captures(after).unwrap();
                [
                    cap[1].parse::<u32>().unwrap(),
                    cap[2].parse::<u32>().unwrap(),
                    cap[3].parse::<u32>().unwrap(),
                    cap[4].parse::<u32>().unwrap(),
                ]
            },
        });
        lines.next();
    }
    Input {
        examples: examples,
        program: lines.map(|line| {
            let cap = re_registers.captures(line).unwrap();
            [
                cap[1].parse::<u32>().unwrap(),
                cap[2].parse::<u32>().unwrap(),
                cap[3].parse::<u32>().unwrap(),
                cap[4].parse::<u32>().unwrap(),
            ]
        }).collect(),
    }
}

fn main() {
    let input = parse(get_input(16, 2018));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
