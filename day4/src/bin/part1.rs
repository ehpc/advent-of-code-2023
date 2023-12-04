use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::{BufRead, Error};

fn main() -> Result<(), Error> {
    let file = File::open("input1.txt")?;
    let sum = io::BufReader::new(file).lines().fold(0, |total, line| {
        let line = line.unwrap();
        let winning_numbers = get_winning_numbers(&line);
        let current_numbers = get_current_numbers(&line);
        let matching_numbers = current_numbers.intersection(&winning_numbers);
        total
            + matching_numbers.fold(0i32, |acc, _| match acc {
                0 => 1,
                _ => acc * 2,
            })
    });
    println!("sum: {:?}", sum);
    Ok(())
}

fn get_winning_numbers(line: &String) -> HashSet<String> {
    let numbers: Vec<String> = line
        .split(':')
        .next_back()
        .unwrap()
        .split('|')
        .next()
        .unwrap()
        .split_whitespace()
        .map(str::to_string)
        .collect();
    HashSet::from_iter(numbers)
}

fn get_current_numbers(line: &String) -> HashSet<String> {
    let numbers: Vec<String> = line
        .split(':')
        .next_back()
        .unwrap()
        .split('|')
        .next_back()
        .unwrap()
        .split_whitespace()
        .map(str::to_string)
        .collect();
    HashSet::from_iter(numbers)
}
