use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::{BufRead, Error};

fn main() -> Result<(), Error> {
    let lines = get_lines();
    let time = parse_number(&lines[0]);
    let distance = parse_number(&lines[1]);
    let range = 1..time;
    let winning_combinations_count = range.fold(0u64, |acc, push_time| {
        let time_left = time - push_time;
        let traveled_distance = time_left * push_time;
        if traveled_distance > distance {
            return acc + 1;
        }
        return acc;
    });
    debug(winning_combinations_count);
    Ok(())
}

fn get_lines() -> Vec<String> {
    let file = File::open("input1.txt").unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

fn parse_number(line: &String) -> u64 {
    let mut line: String = line.split(':')
        .next_back()
        .unwrap()
        .to_string();
    line.retain(|x| !x.is_whitespace());
    return line.parse::<u64>().unwrap();
}

fn debug<T: Debug>(val: T) {
    println!("{:?}", val)
}
