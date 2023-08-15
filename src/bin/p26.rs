// Problem 26: Reciprocal Cycles 

use std::collections::HashMap;

fn reciprocal_cycle_length(number: usize) -> usize {
  let mut seen: HashMap<usize, usize> = HashMap::new();
  let mut dividend = 1;
  let mut quotient: Vec<usize> = vec![];
  let mut counter = 0;

  loop {
    if dividend == 0 {
      break;
    }

    while dividend < number {
      dividend *= 10;
    }

    if seen.contains_key(&dividend) {
      return counter - seen.get(&dividend).unwrap();
    }

    seen.insert(dividend, counter);
    counter += 1;
  
    quotient.push(dividend / number);
    dividend %= number;
  }

  0
}

fn main() {
  let mut result: usize = 0;
  let mut d: usize = 0;
  for number in 2..=1000 {
    let temp = reciprocal_cycle_length(number);
    if result < temp {
      d = number;
      result = temp;
    }
  }

  println!("{} {}", result, d);
}
