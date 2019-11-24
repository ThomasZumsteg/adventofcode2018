use std::collections::{HashMap, HashSet};

use common::get_input;
use common::op_code::{self, Instruction};

struct Input {
    ip: usize,
    program: Vec<Instruction>,
}

fn part1(input: &Input) -> usize {
    let mut registers = [0; 6];
    let opcodes = op_code::new();
    while registers[input.ip] != 28 {
        if let Some(instr) = input.program.get(registers[input.ip]) {
            registers = opcodes[&instr.name](instr.reg, registers);
            registers[input.ip] += 1;
        } else {
            return registers[0]
        }
    }
    registers[3]
}

fn part2(_: &Input) -> usize {
    let mut seen = HashSet::new();
    let mut state = (0, 0);
    let mut steps = 0;
    let mut min_steps: HashMap<usize, usize> = HashMap::new();
    while !seen.contains(&state) {
        seen.insert(state);
        if !min_steps.contains_key(&state.0) {
            min_steps.insert(state.0, steps);
        }
        steps += 1;
        state = update_func(state.0)
    }
    min_steps.into_iter()
        .max_by(|&a, &b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap().0
}

fn update_func(mut reg3: usize) -> (usize, usize) {
    let mut reg5 = 65536 | reg3;
    reg3 = 5557974;
    loop {
        reg3 += reg5 & 255;
        reg3 &= 16777215;
        reg3 *= 65899;
        reg3 &= 16777215;
        if reg5 < 256 {
            break
        }
        reg5 /= 256;
    }
    (reg3, reg5)
}

fn parse(text: String) -> Input {
    let mut first: Option<usize> = None;
    let mut program: Vec<Instruction> = Vec::new();
    for (n, line) in text.trim().split('\n').enumerate() {
        if n != 0 {
            program.push(Instruction::new(line));
        } else {
            first = Some(line.split(' ').skip(1).next().unwrap()
                .parse::<usize>().unwrap());
        }
    }
    Input {
        ip: first.unwrap(),
        program: program,
    }
}

fn main() {
    let input = parse(get_input(21, 2018));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
