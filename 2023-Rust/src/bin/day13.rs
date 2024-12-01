#![allow(unused_variables)]
#![allow(dead_code)]

type Pattern = Vec<Vec<char>>;

#[derive(Eq, PartialEq, Debug)]
enum ReflectionAxis {
    Horizontal(usize),
    Vertical(usize),
    NotFound,
}

fn is_reflection_axis(pattern: &Pattern, axis: &ReflectionAxis, i: usize, j: usize) -> bool {
    match axis {
        ReflectionAxis::Horizontal(_) => {
            for c in 0..pattern[0].len() {
                if pattern[i][c] != pattern[j][c] {
                    return false;
                }
            }

            if i == 0 || j == pattern.len() - 1 {
                return true;
            }
            return is_reflection_axis(pattern, axis, i - 1, j + 1);
        }
        ReflectionAxis::Vertical(_) => {
            for l in 0..pattern.len() {
                if pattern[l][i] != pattern[l][j] {
                    return false;
                }
            }

            if i == 0 || j == pattern[0].len() - 1 {
                return true;
            }
            return is_reflection_axis(pattern, axis, i - 1, j + 1);
        }
        _ => panic!("No axis given!"),
    }
}

fn find_refflection(pattern: &Pattern) -> ReflectionAxis {
    for i in 1..pattern.len() {
        if is_reflection_axis(pattern, &ReflectionAxis::Horizontal(0), i - 1, i) {
            return ReflectionAxis::Horizontal(i);
        }
    }

    for i in 1..pattern[0].len() {
        if is_reflection_axis(pattern, &ReflectionAxis::Vertical(0), i - 1, i) {
            return ReflectionAxis::Vertical(i);
        }
    }

    return ReflectionAxis::NotFound;
}

fn find_refflection_h(pattern: &Pattern, prev: &ReflectionAxis) -> ReflectionAxis {
    for i in 1..pattern.len() {
        if ReflectionAxis::Horizontal(i) == *prev {
            continue;
        }

        if is_reflection_axis(pattern, &ReflectionAxis::Horizontal(0), i - 1, i) {
            return ReflectionAxis::Horizontal(i);
        }
    }

    return ReflectionAxis::NotFound;
}
fn find_refflection_v(pattern: &Pattern, prev: &ReflectionAxis) -> ReflectionAxis {
    for i in 1..pattern[0].len() {
        if ReflectionAxis::Vertical(i) == *prev {
            continue;
        }

        if is_reflection_axis(pattern, &ReflectionAxis::Vertical(0), i - 1, i) {
            return ReflectionAxis::Vertical(i);
        }
    }

    return ReflectionAxis::NotFound;
}

fn replace_char(c: &mut char) {
    if *c == '#' {
        *c = '.';
    } else {
        *c = '#';
    }
}

fn find_refflection_smudge(pattern: &mut Pattern) -> ReflectionAxis {
    let old_pattern = pattern.clone();
    let old_reflection = find_refflection(&old_pattern);

    for l in 0..pattern.len() {
        for c in 0..pattern[0].len() {
            replace_char(&mut pattern[l][c]);

            let reflection_h = find_refflection_h(&pattern, &old_reflection);
            let reflection_v = find_refflection_v(&pattern, &old_reflection);

            if reflection_h != ReflectionAxis::NotFound {
                if reflection_h != old_reflection {
                    return reflection_h;
                }
            }

            if reflection_v != ReflectionAxis::NotFound {
                if reflection_v != old_reflection {
                    return reflection_v;
                }
            }

            replace_char(&mut pattern[l][c]);
        }
    }

    utils::print_mat(&pattern);
    panic!("Unable to find axis!");
}

fn part1(filename: &str) -> usize {
    let patterns = utils::read_matrices(filename);
    let mut sum = 0;

    for p in patterns {
        match find_refflection(&p) {
            ReflectionAxis::Vertical(x) => sum += x,
            ReflectionAxis::Horizontal(x) => sum += x * 100,
            _ => panic!("No axis found"),
        }
    }
    return sum;
}

fn part2(filename: &str) -> usize {
    let patterns = utils::read_matrices(filename);
    let mut sum = 0;

    for mut p in patterns {
        match find_refflection_smudge(&mut p) {
            ReflectionAxis::Vertical(x) => sum += x,
            ReflectionAxis::Horizontal(x) => sum += x * 100,
            _ => panic!("No axis found"),
        }
    }
    return sum;
}

fn main() {
    println!("Part1: {}", part1("inputs/day13.txt"));
    println!("Part2: {}", part2("inputs/day13.txt"));
}

// *====================== Tests ======================* //

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_reflection_axis() {
        let patterns = utils::read_matrices("examples/day13.txt");
        let r = find_refflection(&patterns[0]);
        assert_eq!(r, ReflectionAxis::Vertical(5));

        let patterns = utils::read_matrices("examples/day13.txt");
        let r = find_refflection(&patterns[1]);
        assert_eq!(r, ReflectionAxis::Horizontal(4));
    }

    #[test]
    fn test_part1() {
        let result = part1("examples/day13.txt");
        assert_eq!(result, 405);
    }

    #[test]
    fn test_part2() {
        let result = part2("examples/day13.txt");
        assert_eq!(result, 400);
    }
}
