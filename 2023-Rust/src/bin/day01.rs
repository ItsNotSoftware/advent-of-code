#![allow(unused_variables)]
#![allow(dead_code)]
use utils;

fn get_number(line: &str) -> u32 {
    let numbers_in_line = line.chars().filter(|c| c.is_digit(10)).collect::<String>();

    let len = numbers_in_line.len();

    let first_char = &numbers_in_line[0..1];
    let last_char = &numbers_in_line[len - 1..len];

    let n = format!("{}{}", first_char, last_char);

    return match n.parse::<u32>() {
        Ok(n) => n,
        Err(_) => 0,
    };
}

fn slice_to_number(slice: &str) -> Option<&str> {
    return match slice {
        "0" => Some("0"),
        "1" => Some("1"),
        "2" => Some("2"),
        "3" => Some("3"),
        "4" => Some("4"),
        "5" => Some("5"),
        "6" => Some("6"),
        "7" => Some("7"),
        "8" => Some("8"),
        "9" => Some("9"),
        "zero" => Some("0"),
        "one" => Some("1"),
        "two" => Some("2"),
        "three" => Some("3"),
        "four" => Some("4"),
        "five" => Some("5"),
        "six" => Some("6"),
        "seven" => Some("7"),
        "eight" => Some("8"),
        "nine" => Some("9"),
        _ => None,
    };
}

fn replace_with_number(s: &String, start: usize, end: usize, number: &str) -> String {
    let mut new_s = s.clone();

    new_s.replace_range(start..=start, number);
    new_s.drain(start + 1..=end);

    return new_s;
}

// ! This function was design to replace all numbers insted of just the first and the last.
// ! Althogh this worked in teory it failed on some inputs like "9twone",
// ! because it would read "two" before reading "one" producing "92ne" instead of "9tw1"
// ! For that a dirty fix was implemented, not the prettiest but it works... ¯\_( ͡° ͜ʖ ͡°)_/¯
fn replace_spelled_numbers(s: &String) -> String {
    let mut new_s = s.clone();

    // Get first number
    'outer: for i in 0..new_s.len() {
        for j in i..=new_s.len() {
            let slice = &new_s[i..j]; // Changed to new_s
            let result = slice_to_number(slice);

            match result {
                Some(number) => {
                    new_s = replace_with_number(&new_s, i, j - 1, number);
                    break 'outer;
                }
                None => continue,
            }
        }
    }

    // Get last number
    'outer: for i in (0..new_s.len()).rev() {
        for j in (i..=new_s.len()).rev() {
            let slice = &new_s[i..j]; // Changed to new_s
            let result = slice_to_number(slice);

            match result {
                Some(number) => {
                    new_s = replace_with_number(&new_s, i, j - 1, number);
                    break 'outer;
                }
                None => continue,
            }
        }
    }

    return new_s;
}

fn part1(filename: &str) -> u32 {
    let input = utils::read_lines(filename);
    let mut sum = 0;

    for line in &input {
        sum += get_number(&line);
    }

    return sum;
}

fn part2(filename: &str) -> u32 {
    let input = utils::read_lines(filename);
    let mut sum = 0;

    for line in &input {
        let filtered_line = replace_spelled_numbers(&line);
        sum += get_number(&filtered_line);
    }

    return sum;
}

fn main() {
    println!("Part1: {}", part1("inputs/day01.txt"));
    println!("Part2: {}", part2("inputs/day01.txt"));
}

// ====================== Tests ====================== //

#[cfg(test)]
mod tests {
    use super::*;

    // - - - - - - - - - - Part1 - - - - - - - - - - //
    fn test_get_number(s: &str, answer: u32) {
        let result = get_number(s);
        println!("input: {}, output: {}\n", s, result);
        assert_eq!(result, answer);
    }

    #[test]
    fn test_part1() {
        test_get_number("1abc2", 12);
        test_get_number("pqr3stu8vwx", 38);
        test_get_number("a1b2c3d4e5f", 15);
        test_get_number("treb7uchet", 77);

        let result = part1("examples/day01a.txt");
        assert_eq!(result, 142);
    }

    // - - - - - - - - - - Part1 - - - - - - - - - - //
    #[test]
    fn test_replace_with_number() {
        let test_s = String::from("onetwothree");
        let result = replace_with_number(&test_s, 0, 2, "1");
        assert_eq!(result, "1twothree");

        let result = replace_with_number(&result, 1, 3, "2");
        assert_eq!(result, "12three");

        let result = replace_with_number(&result, 2, 6, "3");
        assert_eq!(result, "123");
    }

    #[test]
    fn test_replace_spelled_numbers() {
        let mut test_s = String::from("5one5twothreea3zero");
        let result = replace_spelled_numbers(&mut test_s);
        assert_eq!(result, "5one5twothreea30");

        let mut test_s = String::from("two1nine");
        let s = replace_spelled_numbers(&mut test_s);
        let result = get_number(&s);
        assert_eq!(result, 29);

        let mut test_s = String::from("eightwothree");
        let s = replace_spelled_numbers(&mut test_s);
        let result = get_number(&s);
        assert_eq!(result, 83);

        let mut test_s = String::from("abcone2threexyz");
        let s = replace_spelled_numbers(&mut test_s);
        let result = get_number(&s);
        assert_eq!(result, 13);

        let mut test_s = String::from("xtwone3four");
        let s = replace_spelled_numbers(&mut test_s);
        let result = get_number(&s);
        assert_eq!(result, 24);

        let mut test_s = String::from("4nineeightseven2");
        let s = replace_spelled_numbers(&mut test_s);
        let result = get_number(&s);
        assert_eq!(result, 42);

        let mut test_s = String::from("zoneight234");
        let s = replace_spelled_numbers(&mut test_s);
        let result = get_number(&s);
        assert_eq!(result, 14);

        let mut test_s = String::from("7pqrstsixteen");
        let s = replace_spelled_numbers(&mut test_s);
        let result = get_number(&s);
        assert_eq!(result, 76);
    }

    #[test]
    fn test_part2() {
        let result = part2("examples/day01b.txt");
        assert_eq!(result, 281);
    }
}
