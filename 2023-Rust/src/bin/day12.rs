#![allow(unused_variables)]
#![allow(dead_code)]

use rayon::prelude::*;

#[derive(Debug)]
struct Record {
    springs: Vec<char>,
    sizes: Vec<u8>,
}

impl Record {
    fn new(line: &str) -> Record {
        let mut line = line.split_whitespace();
        let mut springs: Vec<char> = Vec::new();
        let mut sizes: Vec<u8> = Vec::new();

        if let Some(word) = line.next() {
            word.chars().for_each(|c| springs.push(c));
        }
        if let Some(word) = line.next() {
            word.split(',')
                .filter_map(|num_str| num_str.parse::<u8>().ok())
                .for_each(|n| sizes.push(n));
        }

        return Record { springs, sizes };
    }

    fn is_valid(&self, springs: &[char]) -> bool {
        let broken: String = springs.into_iter().collect();
        let broken: Vec<String> = broken
            .split('.')
            .map(String::from)
            .filter(|s| s != "")
            .collect();

        if broken.len() != self.sizes.len() {
            return false;
        }

        for (i, size) in self.sizes.iter().enumerate() {
            if *size as usize != broken[i].len() {
                return false;
            }
        }

        return true;
    }

    fn count_arrangements(&self, i: usize, springs: &[char]) -> u64 {
        if i == springs.len() {
            if self.is_valid(springs) {
                return 1;
            } else {
                return 0;
            }
        }

        if springs[i] == '?' {
            let mut count = 0;
            let mut modified_springs = springs.to_vec();

            modified_springs[i] = '#';
            count += self.count_arrangements(i + 1, &modified_springs);

            modified_springs[i] = '.';
            count += self.count_arrangements(i + 1, &modified_springs);

            return count;
        }

        return self.count_arrangements(i + 1, springs);
    }

    fn expand(&mut self) {
        let mut s1 = self.springs.clone();
        s1.insert(0, '?');
        let s2 = self.sizes.clone();

        for i in 0..4 {
            self.springs.append(&mut s1.clone());
            self.sizes.append(&mut s2.clone());
        }
    }
}

fn parse_input(filename: &str) -> Vec<Record> {
    let input = utils::read_lines(filename);
    let mut record_list: Vec<Record> = Vec::new();

    input
        .iter()
        .for_each(|line| record_list.push(Record::new(line)));

    return record_list;
}

fn part1(filename: &str) -> u64 {
    let record_list = parse_input(filename);

    return record_list
        .iter()
        .map(|r| {
            let mut springs = r.springs.clone();
            r.count_arrangements(0, &mut springs)
        })
        .sum();
}

fn part2(filename: &str) -> u64 {
    let mut record_list = parse_input(filename);
    record_list.iter_mut().for_each(|r| r.expand());

    record_list
        .par_iter() // Use parallel iterator
        .map(|r| {
            let mut springs = r.springs.clone();
            let count = r.count_arrangements(0, &mut springs);
            println!("Finished {}", count);
            return count;
        })
        .sum()
}

fn main() {
    println!("Part1: {}", part1("inputs/day12.txt"));
    println!("Part2: {}", part2("examples/day12.txt"));
}

// *====================== Tests ======================* //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_arrangements() {
        let r = Record::new("???.### 1,1,3");
        assert_eq!(1, r.count_arrangements(0, &mut r.springs.clone()));

        let r = Record::new(".??..??...?##. 1,1,3");
        assert_eq!(4, r.count_arrangements(0, &mut r.springs.clone()));

        let r = Record::new("?#?#?#?#?#?#?#? 1,3,1,6");
        assert_eq!(1, r.count_arrangements(0, &mut r.springs.clone()));

        let r = Record::new("????.#...#... 4,1,1");
        assert_eq!(1, r.count_arrangements(0, &mut r.springs.clone()));

        let r = Record::new("????.######..#####. 1,6,5");
        assert_eq!(4, r.count_arrangements(0, &mut r.springs.clone()));

        let r = Record::new("?###???????? 3,2,1");
        assert_eq!(10, r.count_arrangements(0, &mut r.springs.clone()));
    }

    #[test]
    fn test_part1() {
        let r = part1("examples/day12.txt");
        assert_eq!(21, r);
    }

    fn test_part2() {
        let r = part2("examples/day12.txt");
        assert_eq!(525152, r);
    }
}
