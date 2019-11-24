use common::get_input;
use common::op_code;

struct Input {
    ip: usize,
    program: Vec<op_code::Instruction>
}

fn factors(num: usize) -> Vec<usize> {
    let mut result = vec![]; 
    for n in 1..((num as f64).sqrt() as usize + 1) {
        if num % n == 0 {
            result.push(n);
            result.push(num / n);
        }
    }
    result
}

fn part1(code: &Input) -> usize {
    let mut registers = [0; 6];
    let opcodes = op_code::new();
    let ip = code.ip;
    while registers[ip] != 1 {
        if let Some(instr) = code.program.get(registers[ip]) {
            registers = opcodes[&instr.name](instr.reg, registers);
            registers[ip] += 1;
        } else {
            return registers[0]
        }
    }
    factors(registers[1]).iter().sum()
}

fn part2(code: &Input) -> usize {
    let mut registers = [0; 6];
    registers[0] = 1;
    let opcodes = op_code::new();
    let ip = code.ip;
    while registers[ip] != 1 {
        if let Some(instr) = code.program.get(registers[ip]) {
            registers = opcodes[&instr.name](instr.reg, registers);
            registers[ip] += 1;
        } else {
            return registers[0]
        }
    }
    factors(registers[1]).iter().sum()
}

fn parse(text: String) -> Input {
    let mut first: Option<usize> = None;
    let mut program: Vec<op_code::Instruction> = vec![];
    for (n, line) in text.trim().split('\n').enumerate() {
        if n == 0 {
            first = Some(line.split(' ').skip(1).next().unwrap()
                 .parse::<usize>().unwrap());
        } else {
            program.push(op_code::Instruction::new(line));
        }
    }
    Input {
        ip: first.unwrap(),
        program: program,
    }
}

fn main() {
    let input = parse(get_input(19, 2018));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
