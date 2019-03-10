extern crate common;

use common::get_input;

type Input = Node;

#[derive(Debug)]
struct Node {
    meta: Vec<u32>,
    children: Vec<Node>,
}

impl Node {
    fn new(values: &mut Iterator<Item=u32>) -> Node {
        let n_children = values.next().unwrap();
        let n_meta = values.next().unwrap();
        let mut root = Node { meta: Vec::new(), children: Vec::new() };
        for _ in 0..n_children {
            root.children.push(Node::new(values));
        }
        for _ in 0..n_meta {
            root.meta.push(values.next().unwrap());
        }
        return root;
    }

    fn sum_meta(&self) -> u32 {
        let child_sum: u32 = self.children.iter().map(|v| v.sum_meta()).sum();
        let meta_sum: u32 = self.meta.iter().sum();
        return meta_sum + child_sum;
    }

    fn value(&self) -> u32 {
        if self.children.is_empty() {
            return self.sum_meta()
        }
        let mut total = 0;
        for &n in &self.meta {
            println!("{:?}, {:?}", n, self.children.get(1 + n as usize));
            if let Some(child) = self.children.get(1 + n as usize) {
                total += child.value();
            }
        }
        return total
    }
}

fn part1(input: &Input) -> u32 {
    input.sum_meta()
}

fn part2(input: &Input) -> u32 {
    input.value()
}

fn parse(input: String) -> Input {
    let mut values = input.trim().split(" ")
        .map(|v| v.parse().unwrap());
    return Node::new(&mut values)
}

fn main() {
    let input = parse(get_input(08, 2018));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
