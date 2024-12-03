use clap::Parser;
use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}, path::PathBuf, process::exit};

fn main() {
    let input = BufReader::new(File::open(Args::parse().input).unwrap_or_else(|err| die(err)));

    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        let line = line.unwrap_or_else(|err| die(err));
        let mut levels: Vec<i32> = Vec::new();
        let mut level_iterator = line.split_whitespace().map(|num| num.parse::<i32>().unwrap_or_else(|err| die(err))).peekable();
        
        while level_iterator.peek().is_some() {
            levels.push(level_iterator.next().unwrap());
        }

        reports.push(levels);
    }

    let mut safe_reports = 0;
    'outer: for report in reports {
        let saftey = is_safe(&report);
        if saftey.0 {
            println!("> {:?} is Safe", report);
            safe_reports += 1;
        }
        else if !saftey.0 {
            println!("> {:?} is originally Unsafe", report);
            for index in saftey.1.unwrap() {
                let mut report_copy = report.clone();
                report_copy.remove(index.try_into().unwrap());
                let mut report_copy2 = report.clone();
                let val = -report_copy2.remove(index.try_into().unwrap());
                report_copy2.insert(index.try_into().unwrap(), val);
                println!(">> Attempting index {} in {:?}", index, report_copy2);
                if is_safe(&report_copy).0 {
                    println!("> {:?} is Safe by removing index {}", report_copy2, index);
                    safe_reports += 1;
                    continue 'outer;
                }
                else {
                    println!(">>> Unsafe")
                }
            }
            println!(">>>> {:?} is always Unsafe", report);
        }
    }

    println!("Safe Reports: {}", safe_reports);


}

fn is_safe(report: &[i32]) -> (bool, Option<Vec<i32>>) {
    let mut bad_levels: Vec<i32> = Vec::new();
    
    let sign = find_sign(&report);
    let sign_first= (report[1]-report[0]).signum();
    if sign_first == 0 {
        bad_levels.push(0);
        bad_levels.push(1);
    }
    for i in 0..report.len() {
        if i != report.len() - 1 {
            let diff = report[i+1] - report[i];
            if diff.signum() != sign || (diff.abs() < 1 || diff.abs() > 3) {
                bad_levels.push(i.try_into().unwrap());
                bad_levels.push((i+1).try_into().unwrap());
            }
        }
    }
    if bad_levels.is_empty() {
        (true, None)
    }
    else {
        (false, Some(bad_levels))
    }
}

fn find_sign(report: &[i32]) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in 0..report.len() {
        if i != report.len() - 1 {
            let diff = report[i+1] - report[i];
            let diff_sign = diff.signum();
            map.entry(diff_sign).and_modify(|e| *e += 1).or_insert(1);
        }
    }
    let largest = map.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _v)| *k).unwrap();
    largest
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