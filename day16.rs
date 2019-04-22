use regex::Regex;
use std::iter::FromIterator;
use std::collections::{HashMap, HashSet};

use common::get_input;

struct Example {
    before: [u32; 4],
    registers: [u32; 4],
    after: [u32; 4],
}

struct Input {
    examples: Vec<Example>,
    program: Vec<[u32; 4]>,
}

mod funcs {
    use std::collections::HashMap;

    type FuncFull = Fn([u32; 4], [u32; 4]) -> [u32; 4];

    fn addr(reg: [u32; 4], val: [u32; 4]) -> [u32; 4] {
        let mut new_val = val.clone();
        new_val[reg[3] as usize] = val[reg[1] as usize] + val[reg[2] as usize];
        new_val
    }

    fn addi(reg: [u32; 4], val: [u32; 4]) -> [u32; 4] {
        let mut new_val = val.clone();
        new_val[reg[3] as usize] = val[reg[1] as usize] + reg[2];
        new_val
    }

    fn mulr(reg: [u32; 4], val: [u32; 4]) -> [u32; 4] {
        let mut new_val = val.clone();
        new_val[reg[3] as usize] = val[reg[1] as usize] * val[reg[2] as usize];
        new_val
    }

    fn muli(reg: [u32; 4], val: [u32; 4]) -> [u32; 4] {
        let mut new_val = val.clone();
        new_val[reg[3] as usize] = val[reg[1] as usize] + reg[2];
        new_val
    }

    fn banr(reg: [u32; 4], val: [u32; 4]) -> [u32; 4] {
        let mut new_val = val.clone();
        new_val[reg[3] as usize] = val[reg[1] as usize] & val[reg[2] as usize];
        new_val
    }

    fn bani(reg: [u32; 4], val: [u32; 4]) -> [u32; 4] {
        let mut new_val = val.clone();
        new_val[reg[3] as usize] = val[reg[1] as usize] & reg[2];
        new_val
    }

    fn borr(reg: [u32; 4], val: [u32; 4]) -> [u32; 4] {
        let mut new_val = val.clone();
        new_val[reg[3] as usize] = val[reg[1] as usize] | val[reg[2] as usize];
        new_val
    }

    fn bori(reg: [u32; 4], val: [u32; 4]) -> [u32; 4] {
        let mut new_val = val.clone();
        new_val[reg[3] as usize] = val[reg[1] as usize] | reg[2];
        new_val
    }

    fn setr(reg: [u32; 4], val: [u32; 4]) -> [u32; 4] {
        let mut new_val = val.clone();
        new_val[reg[3] as usize] = val[reg[1] as usize];
        new_val
    }

    fn seti(reg: [u32; 4], val: [u32; 4]) -> [u32; 4] {
        let mut new_val = val.clone();
        new_val[reg[3] as usize] = reg[1];
        new_val
    }

    fn gtir(reg: [u32; 4], val: [u32; 4]) -> [u32; 4] {
        let mut new_val = val.clone();
        new_val[reg[3] as usize] = if reg[1] > val[reg[2] as usize] { 1 } else { 0 };
        new_val
    }

    fn gtri(reg: [u32; 4], val: [u32; 4]) -> [u32; 4] {
        let mut new_val = val.clone();
        new_val[reg[3] as usize] = if val[reg[1] as usize] > reg[2] { 1 } else { 0 };
        new_val
    }

    fn gtrr(reg: [u32; 4], val: [u32; 4]) -> [u32; 4] {
        let mut new_val = val.clone();
        new_val[reg[3] as usize] = if val[reg[1] as usize] > val[reg[2] as usize] { 1 } else { 0 };
        new_val
    }

    fn eqir(reg: [u32; 4], val: [u32; 4]) -> [u32; 4] {
        let mut new_val = val.clone();
        new_val[reg[3] as usize] = if reg[1] == val[reg[2] as usize] { 1 } else { 0 };
        new_val
    }

    fn eqri(reg: [u32; 4], val: [u32; 4]) -> [u32; 4] {
        let mut new_val = val.clone();
        new_val[reg[3] as usize] = if val[reg[1] as usize] == reg[2] { 1 } else { 0 };
        new_val
    }

    fn eqrr(reg: [u32; 4], val: [u32; 4]) -> [u32; 4] {
        let mut new_val = val.clone();
        new_val[reg[3] as usize] = if val[reg[1] as usize] == val[reg[2] as usize] { 1 } else { 0 };
        new_val
    }

    pub fn make_funcs() -> HashMap<String, &'static FuncFull> {
        let mut collection: HashMap<String, &'static FuncFull>  = HashMap::new();
        collection.insert("addr".to_string(), &addr);
        collection.insert("addi".to_string(), &addi);
        collection.insert("mulr".to_string(), &mulr);
        collection.insert("muli".to_string(), &muli);
        collection.insert("banr".to_string(), &banr);
        collection.insert("bani".to_string(), &bani);
        collection.insert("borr".to_string(), &borr);
        collection.insert("bori".to_string(), &bori);
        collection.insert("setr".to_string(), &setr);
        collection.insert("seti".to_string(), &seti);
        collection.insert("gtir".to_string(), &gtir);
        collection.insert("gtri".to_string(), &gtri);
        collection.insert("gtrr".to_string(), &gtrr);
        collection.insert("eqir".to_string(), &eqir);
        collection.insert("eqri".to_string(), &eqri);
        collection.insert("eqrr".to_string(), &eqrr);
        collection
    }
}

fn part1(input: &Input) -> u32 {
    let mut count = 0;
    let funcs = funcs::make_funcs();
    for test in &input.examples {
        let mut works = 0;
        for func in funcs.values() {
            if test.after == func(test.registers, test.before) {
                works += 1;
            }
        }
        if works >= 3 {
            count += 1;
        }
    }
    count
}

fn part2(input: &Input) -> u32 {
    let mut opcode_mapping: HashMap<u32, HashSet<String>> = HashMap::new();
    let funcs = funcs::make_funcs();
    for op_code in 0u32..(funcs.len() as u32) {
        unimplemented!()
        // opcode_mapping.insert(
        //     op_code,
        //     HashSet::from_iter(funcs.keys()));
    }
    for test in &input.examples {
        for (name, func) in funcs.clone() {
            if test.after != func(test.registers, test.before) {
                // opcode_mapping.get_mut(name).remove(name);
            }
        }
    }
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
