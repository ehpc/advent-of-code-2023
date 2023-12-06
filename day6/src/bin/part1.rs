use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::{BufRead, Error};

fn main() -> Result<(), Error> {
    let lines = get_lines();
    let times = parse_numbers(&lines[0]);
    let distances = parse_numbers(&lines[1]);
    let time_distances = times.iter().zip(distances.iter());
    let all_winning_combinations_count: i32 = time_distances.map(|(&time, &distance)| {
        let range = 1..time;
        let winning_combinations_count = range.fold(0i32, |acc, push_time| {
            let time_left = time - push_time;
            let traveled_distance = time_left * push_time;
            if traveled_distance > distance {
                return acc + 1;
            }
            return acc;
        });
        return winning_combinations_count;
    }).product();
    debug(all_winning_combinations_count);
    Ok(())
}

fn get_lines() -> Vec<String> {
    let file = File::open("input1.txt").unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

fn parse_numbers(line: &String) -> Vec<i32> {
    line.split(':')
        .next_back()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn debug<T: Debug>(val: T) {
    println!("{:?}", val)
}
