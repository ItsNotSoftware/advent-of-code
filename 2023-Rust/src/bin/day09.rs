#![allow(unused_variables)]
#![allow(dead_code)]

fn parse_input(filename: &str) -> Vec<Vec<i32>> {
    let input = utils::read_lines(filename);

    return input
        .iter()
        .map(|s| {
            s.split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect()
        })
        .collect();
}

fn compute_derivative(seq: &Vec<i32>) -> i32 {
    let sum: i32 = seq.iter().sum();
    if sum == 0 {
        return 0;
    }

    let mut d_seq: Vec<i32> = Vec::new();
    for i in 0..seq.len() - 1 {
        d_seq.push(seq[i + 1] - seq[i])
    }

    return d_seq.last().unwrap() + compute_derivative(&d_seq);
}

fn compute_integral(seq: &Vec<i32>) -> i32 {
    let mut d_seq: Vec<i32> = Vec::new();
    for i in 0..seq.len() - 1 {
        d_seq.push(seq[i + 1] - seq[i])
    }

    let sum: i32 = d_seq.iter().sum();
    if sum == 0 {
        return seq[0];
    }

    return seq[0] - compute_integral(&d_seq);
}

fn part1(filename: &str) -> i32 {
    let input = parse_input(filename);

    return input
        .iter()
        .map(|seq| seq.last().unwrap() + compute_derivative(seq))
        .sum();
}

fn part2(filename: &str) -> i32 {
    let input = parse_input(filename);
    return input.iter().map(|seq| compute_integral(seq)).sum();
}

fn main() {
    println!("Part1: {}", part1("inputs/day09.txt"));
    println!("Part2: {}", part2("inputs/day09.txt"));
}

// *====================== Tests ======================* //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("examples/day09.txt");
        assert_eq!(result, 114);
    }

    #[test]
    fn test_part2() {
        let result = part2("examples/day09.txt");
        assert_eq!(result, 2);
    }
}
