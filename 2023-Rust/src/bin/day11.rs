#![allow(unused_variables)]
#![allow(dead_code)]

type Image = Vec<Vec<char>>;

const SCALE_FACTOR: i64 = 1000000;

#[derive(Debug)]
struct Galaxy {
    x: i64,
    y: i64,
}

impl Galaxy {
    fn new(x: usize, y: usize) -> Galaxy {
        return Galaxy {
            x: x as i64,
            y: y as i64,
        };
    }

    fn expand(&mut self, empty_lines: &Vec<usize>, empty_columns: &Vec<usize>) {
        let mut empty_lines_above = 0;
        let mut empty_columns_left = 0;

        for l in empty_lines {
            if (*l as i64) < self.y {
                empty_lines_above += 1;
            }
        }

        for c in empty_columns {
            if (*c as i64) < self.x {
                empty_columns_left += 1;
            }
        }

        self.x += empty_columns_left * SCALE_FACTOR - empty_columns_left;
        self.y += empty_lines_above * SCALE_FACTOR - empty_lines_above;
    }

    fn get_distance(&self, g: &Galaxy) -> u64 {
        if std::ptr::eq(self, g) {
            return 0;
        }

        return ((self.x - g.x).abs() + (self.y - g.y).abs()) as u64;
    }
}

fn get_galaxy_vec(img: &Image) -> Vec<Galaxy> {
    let mut galaxies: Vec<Galaxy> = Vec::new();

    for i in 0..img.len() {
        for j in 0..img[0].len() {
            if img[i][j] == '#' {
                galaxies.push(Galaxy::new(j, i));
            }
        }
    }

    return galaxies;
}

fn expand_space(galaxy: Image) -> Image {
    let mut expanded_galaxy: Image = Vec::new();

    for line in &galaxy {
        expanded_galaxy.push(line.clone());
        if line.iter().all(|c| *c == '.') {
            expanded_galaxy.push(line.clone());
        }
    }

    let transposed: Image = (0..expanded_galaxy[0].len())
        .map(|i| expanded_galaxy.iter().map(|row| row[i]).collect())
        .collect();

    let mut expanded_transposed: Image = Vec::new();
    for col in &transposed {
        expanded_transposed.push(col.clone());
        if col.iter().all(|c| *c == '.') {
            expanded_transposed.push(col.clone());
        }
    }

    expanded_galaxy = (0..expanded_transposed[0].len())
        .map(|i| expanded_transposed.iter().map(|row| row[i]).collect())
        .collect();

    return expanded_galaxy;
}

fn expand_space_opt(g_vec: &mut Vec<Galaxy>, img: &Image) {
    let mut empty_lines: Vec<usize> = Vec::new();
    let mut empty_columns: Vec<usize> = Vec::new();

    for i in 0..img.len() {
        let mut empty = true;
        for j in 0..img[0].len() {
            if img[i][j] != '.' {
                empty = false;
            }
        }
        if empty {
            empty_lines.push(i);
        }
    }

    for j in 0..img[0].len() {
        let mut empty = true;
        for i in 0..img.len() {
            if img[i][j] != '.' {
                empty = false;
                break;
            }
        }
        if empty {
            empty_columns.push(j);
        }
    }

    for g in g_vec {
        g.expand(&empty_lines, &empty_columns);
    }
}

fn part1(filename: &str) -> u64 {
    let image = expand_space(utils::read_chars(filename));
    let g_vec = get_galaxy_vec(&image);
    let mut sum_of_distances = 0;

    for g1 in &g_vec {
        for g2 in &g_vec {
            sum_of_distances += g1.get_distance(&g2);
        }
    }

    return sum_of_distances / 2;
}

fn part2(filename: &str) -> u64 {
    let image = utils::read_chars(filename);
    let mut g_vec = get_galaxy_vec(&image);
    let mut sum_of_distances = 0;

    expand_space_opt(&mut g_vec, &image);
    //dbg!(&g_vec);

    for g1 in &g_vec {
        for g2 in &g_vec {
            sum_of_distances += g1.get_distance(&g2);
        }
    }

    return sum_of_distances / 2;
}

fn main() {
    println!("Part1: {}", part1("inputs/day11.txt"));
    println!("Part2: {}", part2("inputs/day11.txt"));
}

// *====================== Tests ======================* //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("examples/day11.txt");
        assert_eq!(result, 374);
    }
}

// 164000584 - 164000420
