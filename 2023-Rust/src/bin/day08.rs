#![allow(unused_variables)]
#![allow(dead_code)]

use std::cmp;
use std::collections::HashMap;
use utils;

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}
impl Node {
    fn new(left: String, right: String) -> Node {
        return Node { left, right };
    }

    fn get_next_node(&self, dir: char) -> &str {
        return match dir {
            'L' => &self.left,
            'R' => &self.right,
            _ => panic!("Invalid direction"),
        };
    }
}

fn parse_input(filename: &str) -> (Vec<char>, HashMap<String, Node>) {
    let input = utils::read_lines(filename);
    let instructions: Vec<char> = input[0].chars().collect();
    let mut map: HashMap<String, Node> = HashMap::new();

    for i in 2..input.len() {
        let l: Vec<String> = input[i]
            .chars()
            .filter(|&c| !['=', ',', '(', ')'].contains(&c))
            .collect::<String>()
            .split_whitespace()
            .map(String::from)
            .collect();

        map.insert(l[0].clone(), Node::new(l[1].clone(), l[2].clone()));
    }

    return (instructions, map);
}

fn get_starting_nodes(filename: &str) -> Vec<String> {
    let input = utils::read_lines(filename);
    let mut starting_nodes: Vec<String> = vec![];

    for i in 2..input.len() {
        let l: Vec<String> = input[i]
            .chars()
            .filter(|&c| !['=', ',', '(', ')'].contains(&c))
            .collect::<String>()
            .split_whitespace()
            .map(String::from)
            .collect();

        if l[0].ends_with("A") {
            starting_nodes.push(l[0].clone());
        }
    }

    return starting_nodes;
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    } else {
        return gcd(b, a % b);
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    return a / gcd(a, b) * b;
}

fn lcm_vec(numbers: Vec<u64>) -> u64 {
    return numbers
        .iter()
        .fold(1, |acc, &num| lcm(acc, cmp::max(num, 1)));
}

fn part1(filename: &str) -> u32 {
    let (instructions, map) = parse_input(filename);
    let mut current_node: &str = "AAA";
    let mut i: usize = 0;
    let mut steps = 0;

    while current_node != "ZZZ" {
        let dir = instructions[i];
        current_node = map[current_node].get_next_node(dir);
        steps += 1;
        i = (i + 1) % instructions.len();
    }

    return steps;
}

fn part2(filename: &str) -> u64 {
    let mut starting_nodes = get_starting_nodes(&filename);
    let (instructions, map) = parse_input(filename);
    let mut steps: Vec<u64> = vec![];

    for node in &mut starting_nodes {
        let mut i: usize = 0;
        let mut counter: u64 = 0;

        while !node.ends_with("Z") {
            let dir = instructions[i];
            *node = map[&node.to_string()].get_next_node(dir).to_string();

            counter += 1;
            i = (i + 1) % instructions.len();
        }
        steps.push(counter);
    }

    return lcm_vec(steps);
}

fn main() {
    println!("Part1: {}", part1("inputs/day08.txt"));
    println!("Part2: {}", part2("inputs/day08.txt"));
}

// *====================== Tests ======================* //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("examples/day08a.txt");
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part2() {
        let result = part2("examples/day08b.txt");
        assert_eq!(result, 6);
    }
}
