// Problem 22: Names Scores 

use std::{fs, path::Path};

fn read_file_vec(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
  let data = fs::read_to_string(Path::new(filepath))?;
  Ok(data)
}

fn calculate_score(name: String) -> usize {
  name
    .chars()
    .map(|x| x as usize - 64)
    .fold(0, |acc, i| acc + i)
}

fn main() {
  let input = read_file_vec("./src/inputs/p22.txt");
  let input = input.unwrap();
  let names: Vec<&str> = input.split(",").collect::<Vec<&str>>();
  let mut names: Vec<String> = names
    .iter()
    .map(|&x| x.trim_matches(|c| c == '\"').to_string())
    .collect();

  names.sort();

  let mut result: usize = 0;

  for (i, name) in names.into_iter().enumerate() {
    result += (i + 1) * calculate_score(name);
  }

  println!("{:?}", result);
}
