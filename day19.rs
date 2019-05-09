use common::get_input;

struct Instruction {
    name: String,
    reg: [usize; 3],
}

impl Instruction {
    fn new(line: &str) -> Instruction {
        let fields: Vec<&str> = line.split(' ').collect();
        Instruction {
            name: fields[0].to_string(),
            reg: [
                fields[1].parse::<usize>().unwrap(),
                fields[2].parse::<usize>().unwrap(),
                fields[3].parse::<usize>().unwrap(),
            ],
        }
    }
}

mod op_code {
    use std::collections::HashMap;

    type Code = [usize; 3];
    type Val = [usize; 6];
    type Op = Fn(Code, Val) -> Val;

    macro_rules! opcode_map(
        { $($key:ident : $value:expr),+ } => {
            {
                let mut m: HashMap<String, &Op> = HashMap::new();
                $(
                    fn $key(code: Code, reg: Val) -> Val {
                        let mut new_val = reg.clone();
                        new_val[code[2]] = $value(code, reg);
                        new_val
                    }
                    m.insert(stringify!($key).to_string(), &$key);
                )+
                m
            }
        };
    );

    pub fn new() -> HashMap<String, &'static Op> {
        opcode_map! {
            addr: |c: Code, r: Val| r[c[0]] + r[c[1]],
            addi: |c: Code, r: Val| r[c[0]] + c[1],
            mulr: |c: Code, r: Val| r[c[0]] * r[c[1]],
            muli: |c: Code, r: Val| r[c[0]] * c[1],
            banr: |c: Code, r: Val| r[c[0]] & r[c[1]],
            bani: |c: Code, r: Val| r[c[0]] & c[1],
            borr: |c: Code, r: Val| r[c[0]] | r[c[1]],
            bori: |c: Code, r: Val| r[c[0]] | c[1],
            setr: |c: Code, r: Val| r[c[0]],
            seti: |c: Code, _: Val| c[0],
            gtir: |c: Code, r: Val| if c[0] > r[c[1]] { 1 } else { 0 },
            gtri: |c: Code, r: Val| if r[c[0]] > c[1] { 1 } else { 0 },
            gtrr: |c: Code, r: Val| if r[c[0]] > r[c[1]] { 1 } else { 0 },
            eqir: |c: Code, r: Val| if c[0] == r[c[1]] { 1 } else { 0 },
            eqri: |c: Code, r: Val| if r[c[0]] == c[1] { 1 } else { 0 },
            eqrr: |c: Code, r: Val| if r[c[0]] == r[c[1]] { 1 } else { 0 }
        }
    }
}

struct Input {
    ip: usize,
    program: Vec<Instruction>
}


fn part1(code: &Input) -> usize {
    let mut registers = [0; 6];
    let opcodes = op_code::new();
    let ip = code.ip;
    loop {
        if let Some(instr) = code.program.get(registers[ip]) {
            registers = opcodes[&instr.name](instr.reg, registers);
            registers[ip] += 1;
        } else {
            return registers[0]
        }
    }
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
    unimplemented!()
}

fn parse(text: String) -> Input {
    let mut first: Option<usize> = None;
    let mut program: Vec<Instruction> = vec![];
    for (n, line) in text.trim().split('\n').enumerate() {
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
    let input = parse(get_input(19, 2018));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
