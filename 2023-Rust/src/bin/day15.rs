#![allow(unused_variables)]
#![allow(dead_code)]
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone)]
struct LensBox {
    label: Vec<String>,
    focal_lenght: Vec<u8>,
}

impl LensBox {
    fn new() -> LensBox {
        LensBox {
            label: Vec::new(),
            focal_lenght: Vec::new(),
        }
    }

    fn push(&mut self, label: String, focal_lenght: u8) {
        for i in 0..self.label.len() {
            if self.label[i] == label {
                self.focal_lenght[i] = focal_lenght;
                return;
            }
        }

        self.label.push(label);
        self.focal_lenght.push(focal_lenght);
    }

    fn remove(&mut self, label: String) {
        for i in 0..self.label.len() {
            if self.label[i] == label {
                self.label.remove(i);
                self.focal_lenght.remove(i);
                return;
            }
        }
    }
}

fn parse_input(filename: &str) -> Vec<String> {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
        .replace("\n", "")
        .split(',')
        .map(|s| s.to_string())
        .collect()
}

fn hash(s: &String) -> u8 {
    let mut current_value: u32 = 0;

    for c in s.as_bytes() {
        current_value += *c as u32;
        current_value = (current_value * 17) % 256;
    }

    current_value as u8
}

fn part1(input: &Vec<String>) -> u32 {
    let mut result: u32 = 0;

    for s in input {
        result += hash(&s) as u32;
    }

    result
}

fn part2(input: &Vec<String>) -> u64 {
    let mut boxes: Vec<LensBox> = vec![LensBox::new(); 256];

    for s in input {
        let val: Vec<&str> = s.split("=").collect();
        let key = hash(&val[0].to_string().replace("-", "")) as usize;

        match s.chars().last().unwrap() {
            '-' => boxes[key].remove(s.replace("-", "")),
            _ => {
                let x: Vec<&str> = s.split("=").collect();
                let label = x[0];
                let focal_lenght = x[1].parse::<u8>().unwrap();
                boxes[key].push(label.to_string(), focal_lenght);
            }
        }
    }

    let mut total_power = 0;
    for (i, b) in boxes.iter().enumerate() {
        let mut box_power = 0;

        for (j, fl) in b.focal_lenght.iter().enumerate() {
            box_power += (j + 1) * (*fl as usize);
        }

        total_power += box_power * (i + 1);
    }

    total_power as u64
}

fn main() {
    let input = parse_input("inputs/day15.txt");
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

// *====================== Tests ======================* //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_input("examples/day15.txt");
        assert_eq!(part1(&input), 1320);
    }

    #[test]
    fn test_part2() {
        let input = parse_input("examples/day15.txt");
        assert_eq!(part2(&input), 145);
    }
}
