use common::get_input;

fn digits(number: u32) -> Vec<u8> {
    if number == 0 {
        return vec![0];
    }
    let mut result = Vec::new();
    let mut remainer = number;
    while remainer > 0 {
        result.insert(0, (remainer % 10) as u8);
        remainer /= 10;
    }
    result
}

fn part1(rounds: u32) -> String {
    let mut recipies = vec![3u8, 7];
    let mut elf1 = 0;
    let mut elf2 = 1;
    loop {
        let digits = digits((recipies[elf1] + recipies[elf2]) as u32);
        for d in digits {
            recipies.push(d);
            if 10 + rounds <= recipies.len() as u32 {
                return recipies[recipies.len()-10..recipies.len()].iter()
                    .map(|d| d.to_string()).collect();
            }
        }
        elf1 = (recipies[elf1] as usize + 1 + elf1) % recipies.len();
        elf2 = (recipies[elf2] as usize + 1 + elf2) % recipies.len();
    }
}

fn part2(input: u32) -> String {
    let mut recipies = vec![3u8, 7];
    let mut elf1 = 0;
    let mut elf2 = 1;
    let target: Vec<u8> = digits(input);
    loop {
        let digits = digits((recipies[elf1] + recipies[elf2]) as u32);
        for d in digits {
            recipies.push(d);
            if recipies.len() < target.len() { continue }
            let digits = recipies[recipies.len()-target.len()..recipies.len()].to_vec(); 
            if digits == target {
                return (recipies.len() - target.len()).to_string();
            }
        }
        elf1 = (recipies[elf1] as usize + 1 + elf1) % recipies.len();
        elf2 = (recipies[elf2] as usize + 1 + elf2) % recipies.len();
    }
}

fn parse(input: String) -> u32 {
    input.trim().parse::<u32>().unwrap()
}

fn main() {
    let input = parse(get_input(14, 2018));
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
