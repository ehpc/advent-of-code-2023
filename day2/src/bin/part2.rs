
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;
use std::cmp;

const INPUT_FILE: &str = "input1.txt";

fn main() {
  if let Ok(lines) = read_lines(INPUT_FILE) {
    let mut sum = 0;
    for line in lines {
      if let Ok(line) = line {
        let turns = get_turns(&line);
        let maxes = get_max_colors(&turns);
        let power = get_power(&maxes);
        sum += power;
      }
    }
    println!("{:?}", sum);
  }
}

fn get_power(cubes: &(i32, i32, i32)) -> i32 {
  cubes.0 * cubes.1 * cubes.2
}

fn get_max_colors(turns: &Vec<(i32, i32, i32)>) -> (i32, i32, i32) {
  turns.iter().fold((0, 0, 0), |acc, x| {
    (cmp::max(acc.0, x.0), cmp::max(acc.1, x.1), cmp::max(acc.2, x.2))
  })
}

fn get_turns(text: &String) -> Vec<(i32, i32, i32)> {
  let mut results: Vec<(i32, i32, i32)> = vec![];
  let game_part = text.split(':').nth(1).unwrap();
  let turn_strings = game_part.split(';');
  for turn_string in turn_strings {
    let mut colors = (0, 0, 0);
    let colors_with_count = turn_string.split(',');
    for color_with_count in colors_with_count {
      let parts: Vec<&str> = color_with_count.split_whitespace().collect();
      let count = parts[0].parse::<i32>().unwrap();
      let color = parts[1];
      match color {
        "red" => colors.0 = count,
        "green" => colors.1 = count,
        "blue" => colors.2 = count,
        _ => {}
      }
    }
    results.push(colors);
  }
  return results;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}