
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

const INPUT_FILE: &str = "input1.txt";
const RED_CUBES_LIMIT: i32 = 12;
const GREEN_CUBES_LIMIT: i32 = 13;
const BLUE_CUBES_LIMIT: i32 = 14;

fn main() {
  if let Ok(lines) = read_lines(INPUT_FILE) {
    let mut sum = 0;
    for line in lines {
      if let Ok(line) = line {
        let game_id = get_game_id(&line);
        let turns = get_turns(&line);
        let valid = are_turns_valid(&turns);
        if valid {
          sum += game_id;
        }
      }
    }
    println!("{}", sum);
  }
}

fn are_turns_valid(turns: &Vec<(i32, i32, i32)>) -> bool {
  turns.iter().all(|&turn| turn.0 <= RED_CUBES_LIMIT
    && turn.1 <= GREEN_CUBES_LIMIT
    && turn.2 <= BLUE_CUBES_LIMIT)
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

fn get_game_id(text: &String) -> i32 {
  let game_part = text.split(':').next().unwrap();
  let id_part = game_part.split_whitespace().nth(1).unwrap();
  id_part.parse::<i32>().unwrap()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}