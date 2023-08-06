// Problem 21: Amicable Numbers 

use std::collections::HashMap;

fn divisor_sum(number: usize) -> usize {
  let mut result = vec![];

  for i in 1..(number / 2 + 1) {
    if number % i == 0 {
      result.push(i);
    }
  }

  result.iter().fold(0, |acc, x| acc + x)
}

fn extract_amicable_numbers(map: &HashMap<usize, usize>) -> Vec<usize> {
  let mut result = vec![];
  
  for (key, value) in map {
    if *key == *value {
      continue;
    }

    if map.contains_key(value) && key == map.get(value).unwrap() {
      result.push(*key);
    }
  }

  result
}

fn main() {
  let mut map: HashMap<usize, usize> = HashMap::new();

  for i in 1..10001 {
    map.insert(i, divisor_sum(i));
  }

  println!("{:?}", extract_amicable_numbers(&map).iter().fold(0, |acc, x| acc + x));
}

