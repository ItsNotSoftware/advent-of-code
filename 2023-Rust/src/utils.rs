use std::fs::File;
use std::io::{BufRead, BufReader, Read};

pub type Mat<T> = Vec<Vec<T>>;

pub fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect(&format!("[File {} not found!]", filename));
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(text) => lines.push(text),
            Err(err) => eprintln!("Error reading line: {}", err),
        }
    }

    return lines;
}

pub fn read_chars(filename: &str) -> Vec<Vec<char>> {
    let file = File::open(filename).expect("Failed to open file");

    let reader = BufReader::new(file);
    let mut vec: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        if let Ok(line_contents) = line {
            let chars: Vec<char> = line_contents.chars().filter(|&c| c != '\n').collect();
            vec.push(chars);
        }
    }

    return vec;
}

pub fn read_matrix(filename: &str) -> Vec<Vec<u32>> {
    let file = File::open(filename).expect("Failed to open file");

    let reader = BufReader::new(file);
    let mut matrix: Vec<Vec<u32>> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let nums: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        matrix.push(nums);
    }

    matrix
}

pub fn read_file(filename: &str) -> String {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("Error opening file: {}", filename),
    };

    let mut file_contents = String::new();
    if let Err(_) = file.read_to_string(&mut file_contents) {
        panic!("Error reading file: {}", filename);
    }

    return file_contents;
}

pub fn print_mat<T>(mat: &[Vec<T>])
where
    T: std::fmt::Display,
{
    for line in mat {
        for c in line {
            print!("{} ", c);
        }
        println!();
    }
    println!();
}

pub fn read_matrices(filename: &str) -> Vec<Vec<Vec<char>>> {
    let file = File::open(filename).expect(&format!("[File {} not found!]", filename));
    let reader = BufReader::new(file);

    let mut matrices: Vec<Vec<Vec<char>>> = Vec::new();
    let mut current_matrix: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(text) => {
                if text.trim().is_empty() {
                    if !current_matrix.is_empty() {
                        matrices.push(current_matrix.clone());
                        current_matrix.clear();
                    }
                } else {
                    let row: Vec<char> = text.chars().collect();
                    current_matrix.push(row);
                }
            }
            Err(err) => eprintln!("Error reading line: {}", err),
        }
    }

    if !current_matrix.is_empty() {
        matrices.push(current_matrix);
    }

    return matrices;
}

// The print_mat function can be used as it is for displaying the matrices.
