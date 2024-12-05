use clap::Parser;
use std::{fs::File, io::{BufReader, Read}, path::PathBuf, process::exit};
use regex::Regex;

fn main() {
    let mut input = BufReader::new(File::open(Args::parse().input).unwrap_or_else(|err| die(err)));
    let mut program = String::new();
    input.read_to_string(&mut program).unwrap_or_else(|err| die(err));

    let re = Regex::new(r"(do\(\))|(don't\(\))|mul\([0-9]+,[0-9]+\)").unwrap();
    let mut sum = 0;

    let matches: Vec<&str> = re.find_iter(&program).map(|m| m.as_str()).collect();

    let mut enabled = true;

    for m in matches {
        if m == "don't()" {
            enabled = false;
            continue;
        }
        if m == "do()" {
            enabled = true;
            continue;
        }

        if enabled {
            let re2 = Regex::new(r"([0-9]+),([0-9]+)").unwrap();
            let caps = re2.captures(m).unwrap();
            let num1 = caps[1].parse::<i32>().unwrap_or_else(|err| die(err));
            let num2 = caps[2].parse::<i32>().unwrap_or_else(|err| die(err));

            sum += num1*num2;
        }
    }

    println!("Sum: {}", sum);

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