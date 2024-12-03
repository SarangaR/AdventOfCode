use clap::Parser;
use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}, path::PathBuf, process::exit};

fn main() {
    let input = BufReader::new(File::open(Args::parse().input).unwrap_or_else(|err| die(err)));
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    
    for line in input.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => die(err),
        };

        let mut num = line.split_whitespace().map(|num| num.parse::<i32>().unwrap_or_else(|err| die(err)));
        list1.push(num.next().unwrap());
        list2.push(num.next().unwrap());
    }

    list1.sort_unstable();
    list2.sort_unstable();

    let mut distance_score = 0;
    for (num1, num2) in list1.iter().zip(list2.iter()) {
        let dist = (num1 - num2).abs();
        distance_score += dist;
    }

    println!("Distance: {}", distance_score);

    let mut similarity_score = 0;
    let mut map: HashMap<i32, i32> = HashMap::new();
    
    for i in list2 {
        map.entry(i).and_modify(|count| *count += 1).or_insert(1);
    }

    for i in list1 {
        let addition = match map.get(&i) {
            Some(value) => *value*i,
            None => 0
        };

        similarity_score += addition;
    }

    println!("Similarity: {}", similarity_score);

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