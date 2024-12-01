#![allow(unused_variables)]
#![allow(dead_code)]

type Map = Vec<Vec<char>>;
type Visited = Vec<Vec<bool>>;

fn get_next_step(pipe: &char, momentum: [i32; 2]) -> Option<[i32; 2]> {
    return match pipe {
        '|' => Some([momentum[0], 0]),
        '-' => Some([0, momentum[1]]),
        'L' => Some([momentum[1], momentum[0]]),
        'J' => Some([-momentum[1], -momentum[0]]),
        '7' => Some([momentum[1], momentum[0]]),
        'F' => Some([-momentum[1], -momentum[0]]),
        _ => None,
    };
}

fn find_starting_pos(map: &Map) -> [usize; 2] {
    for l in 0..map.len() {
        for c in 0..map[0].len() {
            if map[l][c] == 'S' {
                return [l, c];
            }
        }
    }
    panic!("No starting position found");
}

fn walk(map: &Map, pos: &[usize; 2], dir: &[i32; 2]) -> Option<[usize; 2]> {
    let pos = [pos[0] as i32, pos[1] as i32];
    if (pos[0] as i32) + dir[0] >= 0
        && pos[0] + dir[0] < map.len() as i32
        && pos[1] + dir[1] >= 0
        && pos[0] + dir[0] < map[0].len() as i32
    {
        return Some([(pos[0] + dir[0]) as usize, (pos[1] + dir[1]) as usize]);
    }

    return None;
}

fn mark_path(map: &Map) -> Map {
    let starting_pos = find_starting_pos(&map);
    let first_moves = [[1, 0], [-1, 0], [0, 1], [0, -1]];

    for first_move in first_moves {
        let mut dir = first_move;
        let mut pos;
        let mut new_map = map.clone();

        match walk(&map, &starting_pos, &dir) {
            Some(p) => pos = p,
            None => continue,
        }

        loop {
            new_map[pos[0]][pos[1]] = '#';

            if dir == [0, 0] {
                break;
            }

            if map[pos[0]][pos[1]] == '.' {
                break;
            }
            if map[pos[0]][pos[1]] == 'S' {
                return new_map;
            }

            match get_next_step(&map[pos[0]][pos[1]], dir) {
                Some(d) => dir = d,
                None => break,
            }

            match walk(&map, &pos, &dir) {
                Some(p) => pos = p,
                None => break,
            }
        }
    }

    panic!("No cycle found");
}

fn flood_fill(map: &mut Map, pos: [usize; 2]) {
    let mut stack: Vec<[usize; 2]> = Vec::new();
    stack.push(pos);

    while !stack.is_empty() {
        let pos = stack.pop().unwrap();

        if map[pos[0]][pos[1]] == '#' || map[pos[0]][pos[1]] == 'O' {
            continue;
        }

        map[pos[0]][pos[1]] = 'O';

        if pos[0] > 0 {
            stack.push([pos[0] - 1, pos[1]]);
        }

        if pos[0] < map.len() - 1 {
            stack.push([pos[0] + 1, pos[1]]);
        }

        if pos[1] > 0 {
            stack.push([pos[0], pos[1] - 1]);
        }

        if pos[1] < map[0].len() - 1 {
            stack.push([pos[0], pos[1] + 1]);
        }
    }
}

fn remove_outsiders(map: &mut Map) {
    let border_l = [0, map.len() - 1];
    let border_c = [0, map[0].len() - 1];

    for i in border_l {
        for j in 0..map[0].len() {
            if map[i][j] != '#' || map[i][j] != 'O' {
                flood_fill(map, [i, j]);
            }
        }
    }

    for i in border_c {
        for j in 0..map.len() {
            if map[j][i] != '#' || map[j][i] != 'O' {
                flood_fill(map, [j, i]);
            }
        }
    }
}

fn is_inside(map: &Map, marker: &Map, point: [usize; 2]) -> bool {
    let mut cross_count = 0;
    let i = point[0];

    for j in 0..point[1] {
        if marker[i][j] == '#' && (map[i][j] == '|' || map[i][j] == 'J' || map[i][j] == 'L') {
            cross_count += 1;
        }
    }
    return cross_count % 2 != 0;
}

fn part1(filename: &str) -> u32 {
    let map: Map = utils::read_chars(filename);
    let starting_pos = find_starting_pos(&map);
    let first_moves = [[1, 0], [-1, 0], [0, 1], [0, -1]];
    let mut n_moves;

    for first_move in first_moves {
        let mut dir = first_move;
        let mut pos;

        match walk(&map, &starting_pos, &dir) {
            Some(p) => pos = p,
            None => continue,
        }

        n_moves = 0;

        loop {
            if dir == [0, 0] {
                break;
            }

            if map[pos[0]][pos[1]] == '.' {
                break;
            }
            if map[pos[0]][pos[1]] == 'S' {
                return n_moves / 2 + 1;
            }
            n_moves += 1;

            match get_next_step(&map[pos[0]][pos[1]], dir) {
                Some(d) => dir = d,
                None => break,
            }

            match walk(&map, &pos, &dir) {
                Some(p) => pos = p,
                None => break,
            }
        }
    }

    panic!("No cycle found");
}

fn part2(filename: &str) -> u32 {
    let map: Map = utils::read_chars(filename);
    let mut markers = mark_path(&map);
    remove_outsiders(&mut markers);
    let mut insiders_count = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if markers[i][j] != '#' && markers[i][j] != 'O' {
                if is_inside(&map, &markers, [i, j]) {
                    insiders_count += 1;
                }
            }
        }
    }

    return insiders_count;
}

fn main() {
    println!("Part1: {}", part1("inputs/day10.txt"));
    println!("Part2: {}", part2("inputs/day10.txt"));
}

// *====================== Tests ======================* //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("examples/day10a.txt");
        assert_eq!(result, 8);

        let result = part1("inputs/day10.txt");
        assert_eq!(result, 7097);
    }

    #[test]
    fn test_part2() {
        let result = part2("examples/day10b.txt");
        assert_eq!(result, 10);
    }
}
