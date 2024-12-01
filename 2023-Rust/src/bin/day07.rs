#![allow(unused_variables)]
#![allow(dead_code)]

use utils;

const FIVE_OF_A_KIND: u32 = 6;
const FOUR_OF_A_KIND: u32 = 5;
const FULL_HOUSE: u32 = 4;
const THREE_OF_A_KIND: u32 = 3;
const TWO_PAIR: u32 = 2;
const ONE_PAIR: u32 = 1;
const HIGH_CARD: u32 = 0;

#[derive(Clone, Debug)]
struct Round {
    hand: String,
    bid: u32,
}

impl Round {
    fn new(input: String) -> Round {
        let input: Vec<&str> = input.split_whitespace().collect();

        return Round {
            hand: input[0].to_string(),
            bid: input[1].parse().unwrap(),
        };
    }
    fn get_type_score(&self) -> u32 {
        let mut card_count: Vec<u32> = vec![0; 14];

        for c in self.hand.chars() {
            let score: usize = get_card_score_p1(c) as usize;
            card_count[score] += 1;
        }
        card_count.sort_by(|a, b| b.cmp(a)); // reverse sort

        return match (card_count[0], card_count[1]) {
            (5, 0) => FIVE_OF_A_KIND,
            (4, 1) => FOUR_OF_A_KIND,
            (3, 2) => FULL_HOUSE,
            (3, 1) => THREE_OF_A_KIND,
            (2, 2) => TWO_PAIR,
            (2, 1) => ONE_PAIR,
            _ => HIGH_CARD,
        };
    }

    fn get_type_score_joker(&self) -> u32 {
        let card_count: Vec<u32> = vec![0; 14];
        let mut best_type = HIGH_CARD;
        let cards = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];

        for rcard in cards {
            let mut test_hand = self.hand.clone();
            test_hand = test_hand.replace('J', &rcard.to_string());

            let test_round = Round {
                hand: test_hand,
                bid: 0,
            };
            let hand_type = test_round.get_type_score();

            if hand_type > best_type {
                best_type = hand_type;
            }
        }
        return best_type;
    }
}

fn get_card_score_p1(card: char) -> u32 {
    return match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'J' => 10,
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        _ => 0,
    };
}

fn get_card_score_p2(card: char) -> u32 {
    return match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        'J' => 1,
        _ => 0,
    };
}

fn parse_input(filename: &str) -> Vec<Round> {
    let input = utils::read_lines(filename);
    return input.into_iter().map(|line| Round::new(line)).collect();
}

fn compare_hands(a: &Round, b: &Round) -> std::cmp::Ordering {
    if a.get_type_score() > b.get_type_score() {
        return std::cmp::Ordering::Greater;
    } else if a.get_type_score() < b.get_type_score() {
        return std::cmp::Ordering::Less;
    } else {
        // ==
        for (ca, cb) in a.hand.chars().zip(b.hand.chars()) {
            if get_card_score_p1(ca) > get_card_score_p1(cb) {
                return std::cmp::Ordering::Greater;
            } else if get_card_score_p1(ca) < get_card_score_p1(cb) {
                return std::cmp::Ordering::Less;
            }
        }
    }
    return std::cmp::Ordering::Less;
}

fn compare_hands_joker(a: &Round, b: &Round) -> std::cmp::Ordering {
    let a = a.clone();
    let b = b.clone();

    if a.get_type_score_joker() > b.get_type_score_joker() {
        return std::cmp::Ordering::Greater;
    } else if a.get_type_score_joker() < b.get_type_score_joker() {
        return std::cmp::Ordering::Less;
    } else {
        // ==
        for (ca, cb) in a.hand.chars().zip(b.hand.chars()) {
            if get_card_score_p2(ca) > get_card_score_p2(cb) {
                return std::cmp::Ordering::Greater;
            } else if get_card_score_p2(ca) < get_card_score_p2(cb) {
                return std::cmp::Ordering::Less;
            }
        }
    }
    return std::cmp::Ordering::Less;
}

fn part1(filename: &str) -> u64 {
    let mut game = parse_input(filename);
    let mut winnings: u64 = 0;

    game.sort_by(|a, b| compare_hands(a, b));

    for i in 0..game.len() {
        winnings += (game[i].bid as u64) * (i + 1) as u64
    }
    return winnings;
}

fn part2(filename: &str) -> u64 {
    let mut game = parse_input(filename);
    let mut winnings: u64 = 0;

    game.sort_by(|a, b| compare_hands_joker(a, b));

    for i in 0..game.len() {
        winnings += (game[i].bid as u64) * (i + 1) as u64
    }
    return winnings;
}

fn main() {
    println!("Part1: {}", part1("inputs/day07.txt"));
    println!("Part2: {}", part2("inputs/day07.txt"));
}

// *====================== Tests ======================* //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("examples/day07.txt");
        assert_eq!(result, 6440);

        let result = part1("inputs/day07.txt");
        assert_eq!(result, 253933213);
    }

    #[test]
    fn test_part2() {
        let result = part2("examples/day07.txt");
        assert_eq!(result, 5905);
    }
}
