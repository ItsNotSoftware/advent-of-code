#![allow(unused_variables)]
#![allow(dead_code)]
use utils;

#[derive(Debug)]
struct Card {
    card_number: u32,
    winning_numbers: Vec<u32>,
    hand: Vec<u32>,
}

impl Card {
    fn new(s: String) -> Card {
        let mut parts = s.splitn(2, ':');
        let card_info = parts.next().expect("Card information not found");
        let card_number = card_info
            .trim_start_matches("Card")
            .trim()
            .parse::<u32>()
            .expect("Failed to parse card number");

        let numbers: Vec<Vec<u32>> = parts
            .next()
            .expect("Card numbers not found")
            .split('|')
            .map(|part| {
                part.split_whitespace()
                    .filter_map(|s| s.parse::<u32>().ok())
                    .collect()
            })
            .collect();

        let winning_numbers = numbers[0].clone();
        let hand = numbers[1].clone();

        return Card {
            card_number,
            winning_numbers,
            hand,
        };
    }

    fn get_count(&self) -> u32 {
        let count = self
            .hand
            .iter()
            .filter(|&number| self.winning_numbers.contains(number))
            .count() as u32;

        return count;
    }
}

fn parse_input(filename: &str) -> Vec<Card> {
    let lines = utils::read_lines(filename);
    let mut cards = Vec::new();

    for line in lines {
        let card = Card::new(line);
        cards.push(card);
    }

    return cards;
}

fn part1(filename: &str) -> u32 {
    let mut points = 0;
    let cards = parse_input(filename);

    for card in cards {
        let count = card.get_count();

        if count != 0 {
            points += 2u32.pow(count - 1);
        }
    }

    return points;
}

fn part2(filename: &str) -> u32 {
    let cards = parse_input(filename);
    let mut card_counts = vec![1; cards.len() + 1];
    card_counts.extend(vec![0; cards.len()]);

    for c in &cards {
        let scrap_count = c.get_count();
        let card_index = (c.card_number - 1) as usize;

        let n = card_counts[card_index];

        for i in 1..scrap_count + 1 {
            card_counts[card_index + i as usize] += n;
        }
    }

    let card_counts = &card_counts[0..cards.len()];
    return card_counts.iter().map(|&count| count as u32).sum();
}

fn main() {
    println!("Part1: {}", part1("inputs/day04.txt"));
    println!("Part2: {}", part2("inputs/day04.txt"));
}

// *====================== Tests ======================* //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_count() {
        let card = Card::new("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_owned());
        let count = card.get_count();
        assert_eq!(count, 4);
    }

    #[test]
    fn test_part1() {
        let result = part1("examples/day04a.txt");
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part2() {
        let result = part2("examples/day04b.txt");
        assert_eq!(result, 30);
    }
}
