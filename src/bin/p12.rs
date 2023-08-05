// Problem 12: Highly Divisible Triangular Number 

use std::collections::HashMap;

fn calculate_divisors(num: usize, map: &mut HashMap<usize, usize>) -> usize {
  if map.contains_key(&num) {
    return *map.get(&num).unwrap();
  }

  let mut result = 1;
  let mut temp = num;

  for i in 2..(num / 2 + 1) {
    let mut count = 1;
    while temp % i == 0 {
      temp /= i;
      count += 1;
    }
    result *= count;
  }

  map.insert(num, result);
  result
}

fn main() {
  let mut map: HashMap<usize, usize> = HashMap::new();
  let mut num_divisors = 0;
  let mut i: usize = 2;
  
  while num_divisors <= 500 {
    let n = i / 2;
    if i % 2 == 0 {
      num_divisors = calculate_divisors(2 * n + 1, &mut map) * calculate_divisors(n + 1, &mut map);
    } else {
      num_divisors = calculate_divisors(2 * n + 1, &mut map) * calculate_divisors(n, &mut map);
    }

    i += 1;
  }

  let triangle_num = i * (i + 1) / 2;
  println!("{}", triangle_num);
}


