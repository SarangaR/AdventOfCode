use clap::Parser;
use std::{fs::File, io::{BufRead, BufReader}, path::PathBuf, process::exit};
use regex::Regex;

fn main() {
    let input = BufReader::new(File::open(Args::parse().input).unwrap_or_else(|err| die(err)));
    let mut strings: Vec<String> = Vec::new();
    
    let mut matrix: Vec<String> = Vec::new();
    for line in input.lines() {
        let line = line.unwrap_or_else(|err| die(err));
        matrix.push(line.clone());
        strings.push(line);
    }

    for i in 0..matrix[0].len() {
        let mut vertical = String::new();
        for j in 0..matrix.len() {
            vertical.push(matrix[j].chars().nth(i).unwrap());
        }
        strings.push(vertical);
    }

    let diagonals = find_all_diagonals(&matrix);
    diagonals.iter().for_each(|diagonal| strings.push(diagonal.clone()));

    let mut count = 0;
    let re = Regex::new(r"XMAS|SAMX").unwrap_or_else(|err| die(err));
    for string in strings {
        let matches = find_overlapping_matches(re.as_str(), &string);
        count += matches.len();
    }
    println!("XMAS Count: {}", count);

    let re_new = Regex::new(r"M.M.A.S.S|S.S.A.M.M|M.S.A.M.S|S.M.A.S.M").unwrap_or_else(|err| die(err));

    // loop through 9x9 blocks of the matrix
    let mut xmas_count = 0;
    for i in 0..matrix.len() - 2 {
        for j in 0..matrix[0].len() - 2 {
            let mut block = String::new();
            for k in 0..3 {
                block.push_str(&matrix[i + k][j..j + 3]);
            }
            // let matches = find_overlapping_matches(re_new.as_str(), &block);
            let matches = re_new.find_iter(&block);
            xmas_count += matches.count();
        }
    }

    println!("XMAS Count in 9x9 blocks: {}", xmas_count);
}

fn find_overlapping_matches<'a>(regex: &'a str, text: &'a str) -> Vec<&'a str> {
    let re = Regex::new(regex).unwrap();
    let mut results = Vec::new();
    let mut start = 0;

    while start < text.len() {
        if let Some(mat) = re.find(&text[start..]) {
            let match_start = start + mat.start();
            let match_end = start + mat.end();
            results.push(&text[match_start..match_end]);

            start = match_start + 1;
        } else {
            break;
        }
    }

    results
}

fn find_all_diagonals(matrix: &Vec<String>) -> Vec<String> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut diagonals = Vec::new();

    for start_row in 0..rows {
        let mut diagonal = String::new();
        let mut r = start_row;
        let mut c = 0;
        while r < rows && c < cols {
            diagonal.push(matrix[r].chars().nth(c).unwrap());
            r += 1;
            c += 1;
        }
        diagonals.push(diagonal);
    }
    for start_col in 1..cols {
        let mut diagonal = String::new();
        let mut r = 0;
        let mut c = start_col;
        while r < rows && c < cols {
            diagonal.push(matrix[r].chars().nth(c).unwrap());
            r += 1;
            c += 1;
        }
        diagonals.push(diagonal);
    }

    for start_row in 0..rows {
        let mut diagonal = String::new();
        let mut r = start_row;
        let mut c = cols - 1;
        while r < rows && c < cols {
            diagonal.push(matrix[r].chars().nth(c).unwrap());
            r += 1;
            c = c.wrapping_sub(1);
        }
        diagonals.push(diagonal);
    }
    for start_col in (0..cols - 1).rev() {
        let mut diagonal = String::new();
        let mut r = 0;
        let mut c = start_col;
        while r < rows && c < cols {
            diagonal.push(matrix[r].chars().nth(c).unwrap());
            r += 1;
            c = c.wrapping_sub(1);
        }
        diagonals.push(diagonal);
    }

    diagonals
}

fn die<T: ToString>(err: T) -> ! {
    eprintln!("Error: {}", err.to_string());
    exit(1);
}

#[derive(Parser, Debug)]
struct Args {
    /// Path to the input file
    input: PathBuf,
}