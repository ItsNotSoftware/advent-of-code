#![allow(unused_variables)]
#![allow(dead_code)]

type Platform = Vec<Vec<char>>;

const FLAT_ROCK: char = '#';
const ROUND_ROCK: char = 'O';
const EMPTY: char = '.';

fn move_rock_up(platform: &mut Platform, i: usize, j: usize) -> bool {
    if i == 0 || platform[i - 1][j] != EMPTY {
        return false;
    }

    let mut new_i = i;
    while new_i > 0 && platform[new_i - 1][j] == EMPTY {
        new_i -= 1;
    }
    platform[new_i][j] = ROUND_ROCK;
    platform[i][j] = EMPTY;

    return true;
}

fn move_all_up(platform: &mut Platform) -> bool {
    let mut moved = false;
    for j in 0..platform[0].len() {
        for i in 0..platform.len() {
            if platform[i][j] == ROUND_ROCK {
                if move_rock_up(platform, i, j) {
                    moved = true;
                }
            }
        }
    }
    return moved;
}

fn move_rock_left(platform: &mut Platform, i: usize, j: usize) -> bool {
    if j == 0 || platform[i][j - 1] != EMPTY {
        return false;
    }

    let mut new_j = j;
    while new_j > 0 && platform[i][new_j - 1] == EMPTY {
        new_j -= 1;
    }
    platform[i][new_j] = ROUND_ROCK;
    platform[i][j] = EMPTY;

    return true;
}

fn move_all_left(platform: &mut Platform) -> bool {
    let mut moved = false;
    for i in 0..platform.len() {
        for j in 0..platform[0].len() {
            if platform[i][j] == ROUND_ROCK {
                if move_rock_left(platform, i, j) {
                    moved = true;
                }
            }
        }
    }
    return moved;
}

fn move_rock_down(platform: &mut Platform, i: usize, j: usize) -> bool {
    let mut moved = false;
    let mut new_i = i;
    while new_i < platform.len() - 1 && platform[new_i + 1][j] == EMPTY {
        new_i += 1;
        moved = true;
    }
    if moved {
        platform[new_i][j] = ROUND_ROCK;
        platform[i][j] = EMPTY;
    }
    return moved;
}

fn move_all_down(platform: &mut Platform) -> bool {
    let mut moved = false;
    for j in 0..platform[0].len() {
        for i in (0..platform.len()).rev() {
            if platform[i][j] == ROUND_ROCK {
                if move_rock_down(platform, i, j) {
                    moved = true;
                }
            }
        }
    }
    return moved;
}

fn move_rock_right(platform: &mut Platform, i: usize, j: usize) -> bool {
    let mut moved = false;
    let mut new_j = j;
    while new_j < platform[0].len() - 1 && platform[i][new_j + 1] == EMPTY {
        new_j += 1;
        moved = true;
    }
    if moved {
        platform[i][new_j] = ROUND_ROCK;
        platform[i][j] = EMPTY;
    }
    return moved;
}

fn move_all_right(platform: &mut Platform) -> bool {
    let mut moved = false;
    for i in 0..platform.len() {
        for j in (0..platform[0].len()).rev() {
            if platform[i][j] == ROUND_ROCK {
                if move_rock_right(platform, i, j) {
                    moved = true;
                }
            }
        }
    }
    return moved;
}

fn cycle(platform: &mut Platform) -> bool {
    let mut moved = false;
    moved |= move_all_up(platform);
    moved |= move_all_left(platform);
    moved |= move_all_down(platform);
    moved |= move_all_right(platform);

    return moved;
}

fn calculate_load(platform: &Platform) -> u64 {
    let mut load = 0;

    for (i, line) in platform.iter().enumerate() {
        let round_count = line.iter().filter(|&&c| c == ROUND_ROCK).count();
        load += round_count * (platform.len() - i);
    }

    return load as u64;
}

fn part1(filename: &str) -> u64 {
    let mut platform = utils::read_chars(filename);
    move_all_up(&mut platform);

    return calculate_load(&platform);
}

fn part2(filename: &str) -> u64 {
    let mut platform = utils::read_chars(filename);

    //1000000000

    for _ in 0..1000 {
        if !cycle(&mut platform) {
            break;
        }
    }

    return calculate_load(&platform);
}

fn main() {
    println!("Part1: {}", part1("inputs/day14.txt"));
    println!("Part2: {}", part2("inputs/day14.txt"));
}

// *====================== Tests ======================* //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let r = part1("examples/day14.txt");
        assert_eq!(r, 136);
    }

    #[test]
    fn test_part2() {
        let r = part2("examples/day14.txt");
        assert_eq!(r, 64);
    }
}
