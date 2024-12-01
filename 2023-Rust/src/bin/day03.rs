#![allow(unused_variables)]
#![allow(dead_code)]
use utils;

fn is_symbol(c: char) -> bool {
    return !(c.is_digit(10) || c == '.');
}

fn is_gear(c: char) -> bool {
    return c == '*';
}

fn get_part_number(line: &mut Vec<char>, i: usize) -> u32 {
    let mut number: String = String::new();
    let mut i = i;

    if !line[i].is_digit(10) {
        return 0;
    }

    // get first digit index
    while i > 0 && line[i - 1].is_digit(10) {
        i -= 1;
    }

    for c in line.iter_mut().skip(i) {
        if c.is_digit(10) {
            number.push(*c);
            *c = '.';
        } else {
            break;
        }
    }

    if number.len() == 0 {
        return 0;
    }

    match number.parse::<u32>() {
        Ok(n) => return n,
        Err(c) => {
            println!("[Failed to parse {c}]");
            return 0;
        }
    }
}

fn sum_symbol_neighbors(input: &mut Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    let mut sum = 0;
    let directions = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    for (di, dj) in directions.iter() {
        let ni = i as i32 + di;
        let nj = j as i32 + dj;

        if ni >= 0 && nj >= 0 && (ni as usize) < input.len() && (nj as usize) < input[0].len() {
            sum += get_part_number(&mut input[ni as usize], nj as usize);
        }
    }

    return sum;
}

fn get_gear_ratio(input: &mut Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    let mut gear_ratio = Vec::new();
    let directions = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    for (di, dj) in directions.iter() {
        let ni = i as i32 + di;
        let nj = j as i32 + dj;

        if ni >= 0 && nj >= 0 && (ni as usize) < input.len() && (nj as usize) < input[0].len() {
            let pn = get_part_number(&mut input[ni as usize], nj as usize);
            if pn != 0 {
                gear_ratio.push(pn);
            }
        }
    }

    if gear_ratio.len() == 2 {
        return gear_ratio[0] * gear_ratio[1];
    } else {
        return 0;
    }
}

fn part1(filename: &str) -> u32 {
    let mut input = utils::read_chars(filename);
    let mut sum = 0;

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if is_symbol(input[i][j]) {
                sum += sum_symbol_neighbors(&mut input, i, j);
            }
        }
    }

    return sum;
}

fn part2(filename: &str) -> u32 {
    let mut input = utils::read_chars(filename);
    let mut sum = 0;

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if is_gear(input[i][j]) {
                sum += get_gear_ratio(&mut input, i, j);
            }
        }
    }

    return sum;
}

fn main() {
    println!("Part1: {}", part1("inputs/day03.txt"));
    println!("Part2: {}", part2("inputs/day03.txt"));
}

// *====================== Tests ======================* //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_part_number() {
        let mut s = String::from("test1234jsiad").chars().collect::<Vec<char>>();
        let result = get_part_number(&mut s, 4);
        assert_eq!(result, 1234);

        let mut s = String::from("test1234jsiad").chars().collect::<Vec<char>>();
        let result = get_part_number(&mut s, 5);
        assert_eq!(result, 1234);

        let mut s = String::from("test1234jsiad").chars().collect::<Vec<char>>();
        let result = get_part_number(&mut s, 7);
        assert_eq!(result, 1234);

        let mut s = String::from("12aaa").chars().collect::<Vec<char>>();
        let result = get_part_number(&mut s, 0);
        assert_eq!(result, 12);

        let mut s = String::from("bbb23").chars().collect::<Vec<char>>();
        let result = get_part_number(&mut s, 4);
        assert_eq!(result, 23);

        let mut s = String::from("1aaa").chars().collect::<Vec<char>>();
        let result = get_part_number(&mut s, 0);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_part1() {
        let result = part1("examples/day03.txt");
        assert_eq!(result, 4361);
    }

    #[test]
    fn test_part2() {
        let result = part2("examples/day03.txt");
        assert_eq!(result, 467835);
    }
}
