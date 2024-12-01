#![allow(unused_variables)]
#![allow(dead_code)]

use rayon::prelude::*;

type Map = Vec<Vec<char>>;

#[derive(Debug, Clone)]
struct Beam {
    l: usize,
    c: usize,
    vl: i32,
    vc: i32,
}

impl Beam {
    fn new(l: usize, c: usize, vl: i32, vc: i32) -> Beam {
        Beam { l, c, vl, vc }
    }
}

fn move_beam(beams: &mut Vec<Beam>, energised: &mut Vec<Vec<bool>>, tile: &mut char) {
    let vl_prev = beams[0].vl;
    let vc_prev = beams[0].vc;
    energised[beams[0].l][beams[0].c] = true;

    match tile {
        '/' => {
            beams[0].vl = -vc_prev;
            beams[0].vc = -vl_prev;
        }
        '\\' => {
            beams[0].vl = vc_prev;
            beams[0].vc = vl_prev;
        }
        '|' => {
            if beams[0].vc.abs() == 1 {
                beams.push(Beam::new(beams[0].l, beams[0].c, -1, 0));
                beams[0].vl = 1;
                beams[0].vc = 0;

                *tile = '!'
            }
        }
        '-' => {
            if beams[0].vl.abs() == 1 {
                beams.push(Beam::new(beams[0].l, beams[0].c, 0, -1));
                beams[0].vl = 0;
                beams[0].vc = 1;

                *tile = '_'
            }
        }
        '!' => {
            if beams[0].vc.abs() == 1 {
                beams.remove(0);
                return;
            }
        }
        '_' => {
            if beams[0].vl.abs() == 1 {
                beams.remove(0);
                return;
            }
        }
        '.' => {}
        _ => panic!("Invalid tile"),
    }
    let new_l = (beams[0].l as i32) + beams[0].vl;
    let new_c = (beams[0].c as i32) + beams[0].vc;

    if new_l < 0
        || new_c < 0
        || (new_l as usize) >= energised.len()
        || (new_c as usize) >= energised[0].len()
    {
        beams.remove(0);
    } else {
        beams[0].l = new_l as usize;
        beams[0].c = new_c as usize;
    }
}

fn part1(mut map: Map, b: Beam) -> usize {
    let mut beams = vec![b];
    let mut energised = vec![vec![false; map[0].len()]; map.len()];

    while beams.len() > 0 {
        let l = beams[0].l;
        let c = beams[0].c;
        move_beam(&mut beams, &mut energised, &mut map[l][c]);
    }

    energised
        .iter()
        .flat_map(|line| line.iter())
        .filter(|&&val| val)
        .count()
}

fn part2(map: Map) -> usize {
    let n_lines = map.len();
    let n_columns = map[0].len();
    let mut beams: Vec<Beam> = vec![
        Beam::new(0, 0, 0, 1),
        Beam::new(0, 0, 1, 0),
        Beam::new(n_lines - 1, 0, 0, 1),
        Beam::new(n_lines - 1, 0, -1, 0),
        Beam::new(0, n_columns - 1, 0, -1),
        Beam::new(0, n_columns - 1, 1, 0),
        Beam::new(n_lines - 1, n_columns - 1, -1, 0),
        Beam::new(n_lines - 1, n_columns - 1, 0, -1),
    ];

    for i in 1..(n_lines - 1) {
        beams.push(Beam::new(i, 0, 0, 1));
        beams.push(Beam::new(i, n_columns - 1, 0, -1));
    }

    for i in 1..(n_columns - 1) {
        beams.push(Beam::new(0, i, 1, 0));
        beams.push(Beam::new(n_lines - 1, i, -1, 0));
    }

    beams
        .par_iter()
        .map(|b| part1(map.clone(), b.clone())) // Use map to compute score for each beam
        .max()
        .unwrap_or(0)
}

fn main() {
    let map: Map = utils::read_chars("inputs/day16.txt");

    println!("Part1: {}", part1(map.clone(), Beam::new(0, 0, 0, 1)));
    println!("Part2: {}", part2(map));
}

// *====================== Tests ======================* //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let map: Map = utils::read_chars("examples/day16.txt");
        let r = part1(map, Beam::new(0, 0, 0, 1));
        assert_eq!(r, 46);
    }

    #[test]
    fn test_part2() {
        let map: Map = utils::read_chars("examples/day16.txt");
        let r = part2(map);
        assert_eq!(r, 51);
    }
}
