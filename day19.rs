use common::get_input;

struct Instruction {
    name: String,
    reg: [u8; 3],
}

impl Instruction {
    fn new(line: &str) -> Instruction {
        let fields: Vec<&str> = line.split(' ').collect();
        Instruction {
            name: fields[0].to_string(),
            reg: [
                fields[1].parse::<u8>().unwrap(),
                fields[2].parse::<u8>().unwrap(),
                fields[3].parse::<u8>().unwrap(),
            ],
        }
    }
}

mod OpCode {
    use std::collections::HashMap;

    macro_rules! map(
        { $($key:tt : $value:expr),+ } => {
            {
                let mut m = HashMap::new();
                $(
                    m.insert(key, value);
                )+
                m
            }
        };
    );

    macro_rules! opcode {
        ($name:ident, $func:expr) => (
            fn $name(reg: Reg, val: Reg) -> Reg {
                let mut new_val = val.clone();
                new_val[reg[3] as usize] = $func(val, reg);
                new_val
            }
        )
    }

    pub fn new() -> HashMap<String, ()> {
        map! {
            "addr": opcode!(|c, r| r[c[0]] + r[c[1]]),
            "addi": opcode!(|c, r| r[c[0]] + c[1]),
            "mulr": opcode!(|c, r| r[c[0]] * r[c[1]]),
            "muli": opcode!(|c, r| r[c[0]] * c[1]),
            "banr": opcode!(|c, r| r[c[0]] & r[c[1]]),
            "bani": opcode!(|c, r| r[c[0]] & c[1]),
            "borr": opcode!(|c, r| r[c[0]] | r[c[1]]),
            "bori": opcode!(|c, r| r[c[0]] | c[1]),
            "setr": opcode!(|c, r| r[c[0]]),
            "seti": opcode!(|c, _| r[0]),
            "gtir": opcode!(|c, r| if c[0] > r[c[1]] { 1 } else { 0 }),
            "gtri": opcode!(|c, r| if r[c[0]] > c[1] { 1 } else { 0 }),
            "gtrr": opcode!(|c, r| if r[c[0]] > r[c[1]] { 1 } else { 0 }),
            "eqir": opcode!(|c, r| if c[0] == r[c[1]] { 1 } else { 0 }),
            "eqri": opcode!(|c, r| if r[c[0]] == c[1] { 1 } else { 0 }),
            "eqrr": opcode!(|c, r| if r[c[0]] == r[c[1]] { 1 } else { 0 }),
        }
    }
}

struct Input {
    ip: usize,
    program: Vec<Instruction>
}


fn part1(code: &Input) -> i32 {
    let mut values = [0; 6];
    let opcodes = OpCode::new();
    while values[code.ip] < code.program.len() {
        let reg = code.program[values[code.ip]];
        values = 
        
    }
    unimplemented!()
}

fn part2(code: &Input) -> i32 {
    unimplemented!()
}

fn parse(text: String) -> Input {
    let mut first: Option<usize> = None;
    let mut program: Vec<Instruction> = vec![];
    for (n, line) in text.split('\n').enumerate() {
        if n == 0 {
            first = Some(line.split(' ').skip(1).next().unwrap()
                 .parse::<usize>().unwrap());
        } else {
            program.push(Instruction::new(line));
        }
    }
    Input {
        ip: first.unwrap(),
        program: program,
    }
}

fn main() {
    let input = parse(get_input(29, 2019));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part1(&input));
}
