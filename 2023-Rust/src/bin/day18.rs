#![allow(unused_variables)]
#![allow(dead_code)]

use utils::Mat;

#[derive(Debug)]
struct Instruction {
    direction: char,
    n: u32,
    rgb: u32,
}

impl Instruction {
    fn new(s: &String) -> Self {
        let s: Vec<String> = s
            .replace("(#", "")
            .replace(")", "")
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let direction = s[0].chars().next().unwrap();
        let n = s[1].parse::<u32>().unwrap();
        let rgb = u32::from_str_radix(&s[2], 16).unwrap();

        Instruction { direction, n, rgb }
    }
}

fn find_grid_dimentions(instructions: &Vec<Instruction>) -> (usize, usize) {
    let mut l: i32 = 0;
    let mut c: i32 = 0;
    let mut min_l: i32 = 0;
    let mut min_c: i32 = 0;
    let mut max_l: i32 = 0;
    let mut max_c: i32 = 0;

    for i in instructions {
        let d = match i.direction {
            'U' => (-1, 0),
            'D' => (1, 0),
            'L' => (0, -1),
            'R' => (0, 1),
            _ => (0, 0),
        };

        for _ in 0..i.n {
            l = l as i32 + d.0;
            c = c as i32 + d.1;
        }

        if l < min_l {
            min_l = l;
        }
        if l > max_l {
            max_l = l;
        }
        if c < min_c {
            min_c = c;
        }
        if c > max_c {
            max_c = c;
        }
    }

    (
        ((max_l - min_l) * 3) as usize,
        ((max_c - min_c) * 3) as usize,
    )
}

fn follow_path(instructions: &Vec<Instruction>, grid: &mut Mat<u32>, mut l: usize, mut c: usize) {
    for i in instructions {
        let d = match i.direction {
            'U' => (-1, 0),
            'D' => (1, 0),
            'L' => (0, -1),
            'R' => (0, 1),
            _ => (0, 0),
        };

        for _ in 0..i.n {
            l = (l as i32 + d.0) as usize;
            c = (c as i32 + d.1) as usize;
            grid[l][c] = 1; //? Maybe rgb for p2
        }
    }
}

fn flood_fill(grid: &mut Vec<Vec<u32>>, l: usize, c: usize, val: u32) {
    let mut stack = vec![(l, c)];

    while let Some((x, y)) = stack.pop() {
        if grid[x][y] == 0 {
            grid[x][y] = val;

            if x > 0 {
                stack.push((x - 1, y));
            }
            if x < grid.len() - 1 {
                stack.push((x + 1, y));
            }
            if y > 0 {
                stack.push((x, y - 1));
            }
            if y < grid[0].len() - 1 {
                stack.push((x, y + 1));
            }
        }
    }
}

fn part1(instructions: &Vec<Instruction>, grid: &mut Mat<u32>) -> usize {
    follow_path(instructions, grid, grid.len() / 2, grid[0].len() / 2);

    for l in 0..grid.len() {
        flood_fill(grid, l, 0, 2);
        flood_fill(grid, l, grid[0].len() - 1, 2);
    }

    for c in 0..grid[0].len() {
        flood_fill(grid, 0, c, 2);
        flood_fill(grid, grid.len() - 1, c, 2);
    }

    grid.iter()
        .flat_map(|row| row.iter())
        .filter(|&&value| value != 2)
        .count()
}

fn part2(instructions: &Vec<Instruction>) -> u64 {
    todo!();
}

fn main() {
    let input = utils::read_lines("inputs/day18.txt");
    let instructions: Vec<Instruction> = input.iter().map(|l| Instruction::new(l)).collect();
    let dimensions = find_grid_dimentions(&instructions);
    let grid: Mat<u32> = vec![vec![0; dimensions.1]; dimensions.0];

    println!("Part1: {}", part1(&instructions, &mut grid.clone()));
    println!("Part2: {}", part2(&instructions));
}

// *====================== Tests ======================* //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = utils::read_lines("examples/day18.txt");
        let instructions: Vec<Instruction> = input.iter().map(|l| Instruction::new(l)).collect();
        let dimensions = find_grid_dimentions(&instructions);
        let grid: Mat<u32> = vec![vec![0; dimensions.1]; dimensions.0];
        let r = part1(&instructions, &mut grid.clone());

        assert_eq!(62, r);
    }

    fn test_part2() {
        todo!();
    }
}
