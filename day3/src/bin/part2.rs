use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

const INPUT_FILE: &str = "input1.txt";

fn main() {
  let file = BufReader::new(File::open(INPUT_FILE).unwrap());
  let matrix: Vec<Vec<char>> = file.lines()
    .map(|line| line.unwrap().chars().collect())
    .collect();
  let mut numbers_by_gear_coordinates: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
  let n = matrix.len();
  for i in 0..n {
    let m = matrix[i].len();
    let mut number = 0;
    let mut is_number_valid = false;
    let mut gear_coords: (usize, usize) = (0, 0);
    for j in 0..m {
      let digit = matrix[i][j];
      if digit.is_numeric() {
        number = number * 10 + digit.to_digit(10).unwrap();
        let left = if j as i32 - 1 >= 0 {
          matrix[i][j - 1]
        } else {
          '.'
        };
        let right = if j + 1 < m {
          matrix[i][j + 1]
        } else {
          '.'
        };
        let top = if i as i32 - 1 >= 0 {
          matrix[i - 1][j]
        } else {
          '.'
        };
        let bottom = if i + 1 < n {
          matrix[i + 1][j]
        } else {
          '.'
        };
        let left_top = if i as i32 - 1 >= 0 && j as i32 - 1 >= 0 {
          matrix[i - 1][j - 1]
        } else {
          '.'
        };
        let right_top = if i as i32 - 1 >= 0 && j + 1 < m {
          matrix[i - 1][j + 1]
        } else {
          '.'
        };
        let right_bottom = if i + 1 < n && j + 1 < m {
          matrix[i + 1][j + 1]
        } else {
          '.'
        };
        let left_bottom = if i + 1 < n && j as i32 - 1 >= 0 {
          matrix[i + 1][j - 1]
        } else {
          '.'
        };
        if is_gear_char(left)
          || is_gear_char(right)
          || is_gear_char(top)
          || is_gear_char(bottom) 
          || is_gear_char(left_top)
          || is_gear_char(right_top)
          || is_gear_char(right_bottom)
          || is_gear_char(left_bottom) {
          is_number_valid = true;
          if is_gear_char(left) {
            gear_coords = (i, j - 1);
          } else if is_gear_char(right) {
            gear_coords = (i, j + 1);
          } else if is_gear_char(top) {
            gear_coords = (i - 1, j);
          } else if is_gear_char(bottom) {
            gear_coords = (i + 1, j);
          } else if is_gear_char(left_top) {
            gear_coords = (i - 1, j - 1);
          } else if is_gear_char(right_top) {
            gear_coords = (i - 1, j + 1);
          } else if is_gear_char(right_bottom) {
            gear_coords = (i + 1, j + 1);
          } else if is_gear_char(left_bottom) {
            gear_coords = (i + 1, j - 1);
          } else {
            unreachable!("Invalid gear coordinates");
          }
        }
      }
      if j == m - 1 || !digit.is_numeric() {
        if is_number_valid {
          update_gear_map(number, &mut numbers_by_gear_coordinates, gear_coords);
        }
        number = 0;
        is_number_valid = false;
      }
    }
  }
  let gears_number =calc_gears(numbers_by_gear_coordinates);
  println!("gears_number: {}", gears_number);
}

fn calc_gears(
  numbers_by_gear_coordinates: HashMap<(usize, usize), Vec<u32>>
) -> u32 {
  numbers_by_gear_coordinates.into_iter().fold(0u32, |acc, (_, numbers)| {
    if numbers.len() == 2 {
      acc + numbers[0] * numbers[1]
    } else {
      acc
    }
  })
}

fn update_gear_map(
  number: u32,
  numbers_by_gear_coordinates: &mut HashMap<(usize, usize), Vec<u32>>,
  gear_coords: (usize, usize),
) {
  let mut numbers = numbers_by_gear_coordinates.get(&gear_coords).unwrap_or(&vec![]).to_vec();
  numbers.push(number);
  numbers_by_gear_coordinates.insert(gear_coords, numbers);
}

fn is_gear_char(ch: char) -> bool {
  return ch == '*';
}