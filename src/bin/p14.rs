// Problem 14: Longest Collatz Sequence 

use std::collections::HashMap;

fn collatz(num: usize, map: &mut HashMap<usize, usize>) -> usize {
  if num == 1 {
    return 1;
  }

  if map.contains_key(&num) {
    return *map.get(&num).unwrap();
  }

  let value;
  if num % 2 == 0 {
    value = 1 + collatz(num / 2, map);
  } else {
    value = 1 + collatz(3 * num + 1, map);
  }

  map.insert(num, value);
  *map.get(&num).unwrap()
}

fn main() {
  let mut map = HashMap::new();
  let mut result = 0;
  let mut number = 0;

  for i in 1..1000001 {
    let x = collatz(i, &mut map);
    if x > result {
      result = x;
      number = i;
    }
  }

  println!("{} {}", number, result);
}

