use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_FILE: &str = "input1.txt";

fn main() {
  let file = BufReader::new(File::open(INPUT_FILE).unwrap());
  let matrix: Vec<Vec<char>> = file.lines()
    .map(|line| line.unwrap().chars().collect())
    .collect();
  println!("{:?}", matrix);
  let mut sum = 0;
  let n = matrix.len();
  for i in 0..n {
    let m = matrix[i].len();
    let mut number = 0;
    let mut is_number_valid = false;
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
        if is_special_char(left)
          || is_special_char(right)
          || is_special_char(top)
          || is_special_char(bottom) 
          || is_special_char(left_top)
          || is_special_char(right_top)
          || is_special_char(right_bottom)
          || is_special_char(left_bottom) {
          is_number_valid = true;
        }
      }
      if j == m - 1 || !digit.is_numeric() {
        if is_number_valid {
          sum += number;
        }
        number = 0;
        is_number_valid = false;
      }
    }
  }
  println!("sum: {}", sum);
}

fn is_special_char(ch: char) -> bool {
  return !ch.is_ascii_digit() && ch != '.';
}