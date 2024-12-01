#![allow(unused_variables)]
#![allow(dead_code)]
use std::collections::HashMap;
use utils;

type ColorMap = HashMap<String, u32>;
type GameInfo = Vec<ColorMap>;

fn get_empty_color_map(id: u32) -> ColorMap {
    let mut color_map = HashMap::new();
    color_map.insert(String::from("id"), id);
    color_map.insert(String::from("red"), 0);
    color_map.insert(String::from("green"), 0);
    color_map.insert(String::from("blue"), 0);

    return color_map;
}

fn get_game_info(filename: &str) -> GameInfo {
    let input = utils::read_lines(filename);
    let mut game_info: Vec<ColorMap> = Vec::new();

    for line in input {
        let cleaned_line = line
            .replace(",", "")
            .replace(";", "")
            .replace(":", "")
            .replace("Game", "");

        let mut word_iter = cleaned_line.split_whitespace().map(str::to_owned);
        let game_id = word_iter.next().unwrap().parse::<u32>().unwrap();
        let mut color_map = get_empty_color_map(game_id); // create map

        // Update map with the color counts
        while let (Some(count_str), Some(key)) = (word_iter.next(), word_iter.next()) {
            let count = count_str.parse::<u32>().unwrap();

            // Update the color count if it is greater than the current count for this play
            if color_map[key.as_str()] < count {
                color_map.insert(key.clone(), count);
            }
        }
        game_info.push(color_map);
    }

    return game_info;
}

fn check_valid_game(color_map: &ColorMap) -> bool {
    return color_map["red"] <= 12 && color_map["green"] <= 13 && color_map["blue"] <= 14;
}

fn part1(filename: &str) -> u32 {
    let game_info = get_game_info(filename);
    let mut id_sum = 0;

    for game in game_info {
        if check_valid_game(&game) {
            id_sum += game["id"];
        }
    }

    return id_sum;
}

fn part2(filename: &str) -> u32 {
    let game_info = get_game_info(filename);
    let mut sum = 0;

    for game in game_info {
        sum += game["red"] * game["green"] * game["blue"];
    }

    return sum;
}

fn main() {
    println!("Part1: {}", part1("inputs/day02.txt"));
    println!("Part2: {}", part2("inputs/day02.txt"));
}

// *====================== Tests ======================* //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("examples/day02.txt");
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part2() {
        let result = part2("examples/day02.txt");
        assert_eq!(result, 2286);
    }
}
