#![allow(unused_variables)]
#![allow(dead_code)]
use utils;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Race {
        return Race { time, distance };
    }

    fn get_distance(&self, t_button: u64) -> u64 {
        return t_button * (self.time - t_button);
    }

    fn count_ways_to_beat(&self) -> u64 {
        return (1..=self.time)
            .filter(|&t_hold| self.get_distance(t_hold) > self.distance)
            .count() as u64;
    }
}

fn parse_input1(filename: &str) -> Vec<Race> {
    let input = utils::read_lines(filename);

    let times: Vec<u64> = input[0]
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    let distances: Vec<u64> = input[1]
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    return times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| Race::new(time, distance))
        .collect();
}

fn parse_input2(filename: &str) -> Race {
    let input = utils::read_lines(filename);

    let time: Vec<u64> = input[0]
        .replace(" ", "")
        .split(":")
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    let distance: Vec<u64> = input[1]
        .replace(" ", "")
        .split(":")
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    return Race::new(time[0], distance[0]);
}

fn part1(filename: &str) -> u64 {
    let races = parse_input1(filename);
    let mut result = 1;

    for race in races {
        result *= race.count_ways_to_beat();
    }

    return result;
}

fn part2(filename: &str) -> u64 {
    let race = parse_input2(filename);
    let result = race.count_ways_to_beat();

    return result;
}

fn main() {
    println!("Part1: {}", part1("inputs/day06.txt"));
    println!("Part2: {}", part2("inputs/day06.txt"));
}

// *====================== Tests ======================* //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("examples/day06.txt");
        assert_eq!(result, 288);
    }

    #[test]
    fn test_part2() {
        let result = part2("examples/day06.txt");
        assert_eq!(result, 71503);
    }
}
