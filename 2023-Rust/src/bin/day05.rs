#![allow(unused_variables)]
#![allow(dead_code)]
use utils;

const DEST: usize = 0;
const SRC: usize = 1;
const RANGE: usize = 2;

#[derive(Debug)]
struct MapTo {
    range_maps: Vec<[u64; 3]>,
}

impl MapTo {
    fn new() -> MapTo {
        return MapTo {
            range_maps: Vec::new(),
        };
    }

    fn push_line(&mut self, line: String) {
        let values: Vec<u64> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();

        if values.len() == 3 {
            self.range_maps
                .push([values[DEST], values[SRC], values[RANGE]]);
        }
    }

    fn src_to_dest(&self, src: u64) -> u64 {
        for rm in &self.range_maps {
            if src >= rm[SRC] && src < rm[SRC] + rm[RANGE] {
                return rm[DEST] + (src - rm[SRC]);
            }
        }

        return src;
    }
}

fn parse_input(filename: &str) -> (Vec<u64>, Vec<MapTo>) {
    let input = utils::read_lines(filename);

    let seeds: Vec<u64> = input[0]
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    let mut mapto_vec: Vec<MapTo> = Vec::new();
    let mut current_mapto = MapTo::new();

    for line in input[2..].iter() {
        if line.trim().is_empty() {
            mapto_vec.push(current_mapto);
            current_mapto = MapTo::new();
            continue;
        }

        current_mapto.push_line(line.clone());
    }

    if !current_mapto.range_maps.is_empty() {
        mapto_vec.push(current_mapto);
    }

    return (seeds, mapto_vec);
}

fn print_iter_n(seeds: Vec<u64>) {
    let sum: u64 = seeds
        .iter()
        .enumerate()
        .filter(|&(i, _)| i % 2 == 1)
        .map(|(_, &val)| val)
        .sum();

    println!("[Total number of iterations = {}]", sum);
}

fn part1(filename: &str) -> u64 {
    let (seeds, mapto_vec) = parse_input(filename);
    let mut lowest_location = u64::MAX;

    for seed in seeds {
        let mut curr_number = seed;

        for mapping in &mapto_vec {
            curr_number = mapping.src_to_dest(curr_number);
        }

        if curr_number < lowest_location {
            lowest_location = curr_number;
        }
    }

    return lowest_location;
}

fn part2(filename: &str) -> u64 {
    let (seeds, mapto_vec) = parse_input(filename);
    let mut lowest_location = u64::MAX;
    let mut counter: u64 = 0;

    for i in (0..seeds.len()).step_by(2) {
        for seed in seeds[i]..(seeds[i] + seeds[i + 1]) {
            let mut curr_number = seed;

            for mapping in &mapto_vec {
                curr_number = mapping.src_to_dest(curr_number);
            }

            if curr_number < lowest_location {
                lowest_location = curr_number;
            }
        }
        println!("Finished: {}", counter);
        counter += 1;
    }

    return lowest_location;
}

fn main() {
    println!("Part1: {}", part1("inputs/day05.txt"));
    println!("Part2: {}", part2("inputs/day05.txt"));
}

// *====================== Tests ======================* //

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_mapping() {
        let (seeds, mapto_vec) = parse_input("examples/day05.txt");
        let soil_n = mapto_vec[0].src_to_dest(79);
        assert_eq!(soil_n, 81);

        let soil_n = mapto_vec[0].src_to_dest(14);
        assert_eq!(soil_n, 14);

        let soil_n = mapto_vec[0].src_to_dest(55);
        assert_eq!(soil_n, 57);

        let soil_n = mapto_vec[0].src_to_dest(13);
        assert_eq!(soil_n, 13);

        let answers: Vec<u64> = vec![82, 43, 86, 35];
        for (seed, answer) in seeds.iter().zip(answers.iter()) {
            let mut curr_number = *seed;

            for mappig in &mapto_vec {
                curr_number = mappig.src_to_dest(curr_number);
            }

            assert_eq!(curr_number, *answer);
        }
    }

    #[test]
    fn test_part1() {
        let result = part1("examples/day05.txt");
        assert_eq!(result, 35);
    }

    #[test]
    fn test_part2() {
        let result = part2("examples/day05.txt");
        assert_eq!(result, 46);
    }
}
