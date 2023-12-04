use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io;
use std::io::{BufRead, Error};
use std::ops::Range;

fn main() -> Result<(), Error> {
    let file = File::open("input1.txt")?;
    let lines: Vec<(usize, String)> = io::BufReader::new(file)
        .lines()
        .enumerate()
        .map(|(index, line)| {
          (index, line.unwrap())
        })
        .collect();
    let cards_count_by_line_index: HashMap<usize, usize> = lines.iter()
      .fold(HashMap::new(), |mut acc, (index, line)| {
        let winning_numbers = get_winning_numbers(&line);
        let current_numbers = get_current_numbers(&line);
        let matching_numbers = current_numbers.intersection(&winning_numbers);
        let cards_count = matching_numbers.count();
        acc.insert(*index, cards_count);
        acc
      });
    let size = lines.len();
    let sum = recur(&lines, 0..size, &cards_count_by_line_index);
    println!("sum: {:?}", sum);
    Ok(())
}

fn recur(lines: &Vec<(usize, String)>, range: Range<usize>, cards_count_by_line_index: &HashMap<usize, usize>) -> usize {
    lines[range].iter().fold(0usize, |total, (index, _)| {
        let cards_count = *cards_count_by_line_index.get(index).unwrap();
        if cards_count > 0 {
          let lower_bound = index + 1;
          let upper_bound = index + cards_count + 1;
          let card_indexes = lower_bound..upper_bound;
          total + 1 + recur(lines, card_indexes, cards_count_by_line_index)
        } else {
          total + 1
        }
    })
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
