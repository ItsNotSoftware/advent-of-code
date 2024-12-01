#![allow(unused_variables)]
#![allow(dead_code)]

use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}
impl Part {
    fn new(line: String) -> Self {
        let re = Regex::new(r"x=(\d+),m=(\d+),a=(\d+),s=(\d+)").unwrap();
        let caps = re.captures(&line).ok_or("Invalid format").unwrap();

        Part {
            x: u32::from_str(&caps[1]).unwrap(),
            m: u32::from_str(&caps[2]).unwrap(),
            a: u32::from_str(&caps[3]).unwrap(),
            s: u32::from_str(&caps[4]).unwrap(),
        }
    }

    fn get(&self, c: char) -> u32 {
        match c {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!("No struct member {c}!"),
        }
    }

    fn sum(&self) -> u64 {
        (self.x + self.m + self.a + self.s) as u64
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    cat: Vec<char>,
    cmp: Vec<fn(u32, u32) -> bool>,
    threshold: Vec<u32>,
    dests: Vec<String>,
}
impl Workflow {
    fn new(line: String) -> Self {
        let mut cat: Vec<char> = Vec::new();
        let mut cmp: Vec<fn(u32, u32) -> bool> = Vec::new();
        let mut threshold: Vec<u32> = Vec::new();
        let mut dests: Vec<String> = Vec::new();

        let members: Vec<String> = line
            .replace("}", "")
            .split(",")
            .map(|s| s.to_string())
            .collect();

        for i in 0..(members.len() - 1) {
            cat.push(members[i].chars().nth(0).unwrap());

            match members[i].chars().nth(1).unwrap() {
                '>' => cmp.push(greater_than),
                '<' => cmp.push(less_than),
                _ => panic!("cmp function invalid!"),
            }

            let end = members[i].find(":").unwrap();
            let n = &members[i][2..end].parse::<u32>().unwrap();
            threshold.push(*n);
            dests.push(members[i][(end + 1)..].to_owned());
        }
        dests.push(members.last().unwrap().to_owned().to_owned());

        Workflow {
            cat,
            cmp,
            threshold,
            dests,
        }
    }

    fn apply(&self, p: &Part) -> String {
        for i in 0..self.cat.len() {
            let (cat, cmp, threshold, dest) = self.get_index(i);
            let c = p.get(cat);

            if cmp(c, threshold) {
                return dest;
            }
        }

        self.dests.last().unwrap().clone()
    }

    fn get_index(&self, i: usize) -> (char, fn(u32, u32) -> bool, u32, String) {
        (
            self.cat[i],
            self.cmp[i],
            self.threshold[i],
            self.dests[i].clone(),
        )
    }
}

fn parse_input(filename: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();
    let mut is_workflow: bool = true;

    let lines = utils::read_lines(filename);
    for l in &lines {
        if l.len() == 0 {
            is_workflow = false;
            continue;
        }

        if is_workflow {
            let members: Vec<&str> = l.split("{").collect();
            let k = members[0].to_owned();
            let v = Workflow::new(members[1].to_owned());
            workflows.insert(k, v);
        } else {
            parts.push(Part::new(l.to_owned()));
        }
    }

    (workflows, parts)
}

fn greater_than(a: u32, b: u32) -> bool {
    a > b
}

fn less_than(a: u32, b: u32) -> bool {
    a < b
}

fn count_accepted(
    workflows: &HashMap<String, Workflow>,
    wf_key: String,
    ranges: HashMap<char, (u32, u32)>,
) -> u64 {
    if wf_key == "R" {
        return 0;
    }
    if wf_key == "A" {
        let mut product: u64 = 1;
        for (_, (min, max)) in ranges {
            product *= (max - min + 1) as u64;
        }
        return product;
    }

    let mut ranges = ranges.clone();
    let mut total = 0;
    let mut ranges_remaining = true;

    for i in 0..workflows[&wf_key].cat.len() {
        let (cat, cmp, threshold, dest) = workflows[&wf_key].get_index(0);
        let (min, max) = ranges[&cat];

        // <
        let t: (u32, u32);
        let f: (u32, u32);
        if cmp(1, 2) {
            t = (min, threshold - 1);
            f = (threshold, max);
        } else {
            t = (threshold + 1, max);
            f = (min, threshold);
        }
        if t.0 <= t.1 {
            let mut new_ranges = ranges.clone();
            new_ranges.insert(cat, t);

            total += count_accepted(workflows, dest, new_ranges);
        }
        if f.0 <= f.1 {
            ranges.insert(cat, f);
        } else {
            ranges_remaining = false;
            break;
        }
    }

    if ranges_remaining {
        total += count_accepted(workflows, wf_key, ranges);
    }

    total
}

fn part1(workflows: HashMap<String, Workflow>, parts: Vec<Part>) -> u64 {
    let mut total = 0;

    'outer: for p in parts {
        let mut next_wf = "in".to_owned();

        loop {
            let w = &workflows[&next_wf];
            next_wf = w.apply(&p);

            if next_wf == "A" {
                total += p.sum();
                continue 'outer;
            } else if next_wf == "R" {
                continue 'outer;
            }
        }
    }

    total
}

fn part2(workflows: HashMap<String, Workflow>) -> u64 {
    let mut ranges = HashMap::new();
    ranges.insert('x'.to_owned(), (1, 4000));
    ranges.insert('m'.to_owned(), (1, 4000));
    ranges.insert('a'.to_owned(), (1, 4000));
    ranges.insert('s'.to_owned(), (1, 4000));

    count_accepted(&workflows, "in".to_owned(), ranges)
}

fn main() {
    let (workflows, parts) = parse_input("inputs/day19.txt");

    println!("Part1: {}", part1(workflows.clone(), parts.clone()));
    println!("Part2: {}", part2(workflows.clone()));
}

// *====================== Tests ======================* //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let (workflows, parts) = parse_input("examples/day19.txt");
        let r = part1(workflows, parts);
        assert_eq!(r, 19114);
    }

    #[test]
    fn test_part2() {
        let (workflows, _) = parse_input("examples/day19.txt");
        let r = part2(workflows);
        assert_eq!(r, 167409079868000);
    }
}
