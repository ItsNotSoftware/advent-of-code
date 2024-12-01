#![allow(unused_variables)]
#![allow(dead_code)]

use std::collections::VecDeque;

type Mat<T> = Vec<Vec<T>>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None, // Added to handle the initial state
}

#[derive(Debug, Clone)]
struct NodeInfo {
    cost: u32,
    consecutive_steps: u32, // Track the count of consecutive steps
    direction: Direction,   // Track the current direction
    prev: Option<[usize; 2]>,
    visited: bool,
}
impl NodeInfo {
    fn new() -> NodeInfo {
        NodeInfo {
            cost: std::u32::MAX,
            consecutive_steps: 0,
            direction: Direction::None,
            prev: None,
            visited: false,
        }
    }
}

fn get_neighbors(map: &Mat<u32>, coords: &[usize; 2]) -> (Vec<[usize; 2]>, Vec<Direction>) {
    let mut neighbors: Vec<[usize; 2]> = Vec::new();
    let mut directions: Vec<Direction> = Vec::new();

    if coords[0] > 0 {
        neighbors.push([coords[0] - 1, coords[1]]);
        directions.push(Direction::Up);
    }

    if coords[0] < map.len() - 1 {
        neighbors.push([coords[0] + 1, coords[1]]);
        directions.push(Direction::Down);
    }

    if coords[1] > 0 {
        neighbors.push([coords[0], coords[1] - 1]);
        directions.push(Direction::Left);
    }

    if coords[1] < map[0].len() - 1 {
        neighbors.push([coords[0], coords[1] + 1]);
        directions.push(Direction::Right);
    }

    (neighbors, directions)
}

fn heat_loss(map: &Mat<u32>) -> u32 {
    let mut queue: VecDeque<[usize; 2]> = VecDeque::new();
    let mut info: Mat<NodeInfo> = vec![vec![NodeInfo::new(); map[0].len()]; map.len()];
    info[0][0].cost = 0;
    info[0][0].visited = true;
    queue.push_back([0, 0]);

    while let Some(src) = queue.pop_front() {
        info[src[0]][src[1]].visited = true; // Mark as visited here
        let (neighbors, directions) = get_neighbors(map, &src);

        for (n, d) in neighbors.iter().zip(directions.iter()) {
            let new_cost = info[src[0]][src[1]].cost + map[n[0]][n[1]];

            let (consecutive_steps, new_direction) = if d == &info[src[0]][src[1]].direction {
                (info[src[0]][src[1]].consecutive_steps + 1, d.clone())
            } else {
                (1, d.clone())
            };

            if consecutive_steps >= 2
                || (info[n[0]][n[1]].visited && new_cost >= info[n[0]][n[1]].cost)
            {
                continue;
            }

            info[n[0]][n[1]].cost = new_cost;
            info[n[0]][n[1]].consecutive_steps = consecutive_steps;
            info[n[0]][n[1]].direction = new_direction;
            info[n[0]][n[1]].prev = Some(src);
            queue.push_back(*n);
        }
    }
    trace_path(&info, map, &[map.len() - 1, map[0].len() - 1]);

    info[map.len() - 1][map[0].len() - 1].cost
}

fn trace_path(info: &Mat<NodeInfo>, map: &Mat<u32>, dest: &[usize; 2]) {
    let mut path_map = vec![vec!['.'; map[0].len()]; map.len()];
    let mut current = Some(*dest);

    while let Some(coords) = current {
        path_map[coords[0]][coords[1]] = '#';
        current = info[coords[0]][coords[1]].prev;
    }

    // Mark the start position
    path_map[0][0] = 'S';

    for (i, row) in path_map.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            if val == '#' || val == 'S' {
                print!("{} ", val);
            } else {
                print!("{} ", map[i][j]);
            }
        }
        println!();
    }
}

fn part1(map: &Mat<u32>) -> u32 {
    heat_loss(map)
}

fn part2(map: Mat<u32>) -> u64 {
    todo!();
}

fn main() {
    let map: Mat<u32> = utils::read_matrix("inputs/day17.txt");

    println!("Part1: {}", part1(&map));
    println!("Part2: {}", part2(map));
}

// *====================== Tests ======================* //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let map: Mat<u32> = utils::read_matrix("examples/day17.txt");
        utils::print_mat(&map);
        let r = part1(&map);
        assert_eq!(r, 102);
    }

    fn test_part2() {
        todo!();
    }
}
