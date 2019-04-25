use regex::Regex;
use std::iter::FromIterator;
use std::collections::{HashMap, HashSet};

use common::get_input;

type Reg = [u32; 4];

struct Example {
    before: Reg,
    registers: Reg,
    after: Reg,
}

struct Input {
    examples: Vec<Example>,
    program: Vec<Reg>,
}

mod funcs {
    use super::Reg;
    use std::collections::HashMap;

    macro_rules! opcode {
        ($name:ident, $func:expr) => (
            fn $name(reg: Reg, val: Reg) -> Reg {
                let mut new_val = val.clone();
                new_val[reg[3] as usize] = $func(val, reg);
                new_val
            }
        )
    }

    opcode!(addr, |val: Reg, reg: Reg| val[reg[1] as usize] + val[reg[2] as usize]);
    opcode!(addi, |val: Reg, reg: Reg| val[reg[1] as usize] + reg[2]);
    opcode!(mulr, |val: Reg, reg: Reg| val[reg[1] as usize] * val[reg[2] as usize]);
    opcode!(muli, |val: Reg, reg: Reg| val[reg[1] as usize] * reg[2]);
    opcode!(banr, |val: Reg, reg: Reg| val[reg[1] as usize] & val[reg[2] as usize]);
    opcode!(bani, |val: Reg, reg: Reg| val[reg[1] as usize] & reg[2]);
    opcode!(borr, |val: Reg, reg: Reg| val[reg[1] as usize] | val[reg[2] as usize]);
    opcode!(bori, |val: Reg, reg: Reg| val[reg[1] as usize] | reg[2]);
    opcode!(setr, |val: Reg, reg: Reg| val[reg[1] as usize]);
    opcode!(seti, |_: Reg, reg: Reg| reg[1]);
    opcode!(gtir, |val: Reg, reg: Reg| if reg[1] > val[reg[2] as usize] { 1 } else { 0 });
    opcode!(gtri, |val: Reg, reg: Reg| if val[reg[1] as usize] > reg[2] { 1 } else { 0 });
    opcode!(gtrr, |val: Reg, reg: Reg| if val[reg[1] as usize] > val[reg[2] as usize] { 1 } else { 0 });
    opcode!(eqir, |val: Reg, reg: Reg| if reg[1] == val[reg[2] as usize] { 1 } else { 0 });
    opcode!(eqri, |val: Reg, reg: Reg| if val[reg[1] as usize] == reg[2] { 1 } else { 0 });
    opcode!(eqrr, |val: Reg, reg: Reg| if val[reg[1] as usize] == val[reg[2] as usize] { 1 } else { 0 });

    type FuncFull = Fn(Reg, Reg) -> Reg;

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
        if 3 >= funcs.values().fold(0, |acc, func| 
                if test.after == func(test.registers, test.before) { acc + 1 }
                else { acc }) {
            count += 1;
        }
    }
    count
}

fn part2(input: &Input) -> u32 {
    let mut opcode_mapping: HashMap<u32, HashSet<String>> = HashMap::new();
    let funcs = funcs::make_funcs();
    for op_code in 0u32..(funcs.len() as u32) {
        opcode_mapping.insert(
            op_code,
            HashSet::from_iter(funcs.keys().map(|k| k.clone())));
    }
    for test in &input.examples {
        for (name, func) in funcs.clone() {
            if test.after != func(test.registers, test.before) {
                opcode_mapping.get_mut(&test.registers[0]).unwrap().remove(&name);
            }
        }
    }
    let mut opcode: HashMap<u32, String> = HashMap::new();
    loop {
        let to_remove: Vec<(u32, String)> = opcode_mapping.iter()
            .filter_map(|(k, v)|
                if v.len() == 1 {
                    Some((*k, v.iter().next().unwrap().clone()))
                } else {
                    None
            }).collect();
        if to_remove.is_empty() {
            break
        }
        for (code, value) in to_remove {
            for values in opcode_mapping.values_mut() {
                values.remove(&value);
            }
            opcode.insert(code, value);
        }
        opcode_mapping.retain(|_, v| v.len() > 0);
    }
    let mut values = [0, 0, 0, 0];
    for step in &input.program {
        let instr = &opcode[&step[0]];
        let func = funcs[instr];
        values = func(*step, values);
    }
    return values[0]
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
