// Problem 17: Number Letter Counts

use std::collections::HashMap;

type NumReadableMap<'a> = HashMap::<usize, &'a str>;

fn encode(num: usize, ones: &NumReadableMap, tens: &NumReadableMap, orders: &NumReadableMap) -> String {
  match num {
    0..=19 => ones.get(&num).unwrap().to_string(),
    20..=99 => {
      let upper = tens.get(&((num / 10) * 10)).unwrap().to_string();
      match num % 10 {
        0 => upper,
        lower => format!("{} {}", upper, encode(lower, ones, tens, orders)),
      }
    }
    100..=999 => {
      let name = orders.get(&100).unwrap();
      match (num / 100, num % 100) {
        (upper, 0) => format!("{} {}", encode(upper, ones, tens, orders), name),
        (upper, lower) => {
          format!("{} {} and {}", encode(upper, ones, tens, orders), name, encode(lower, ones, tens, orders))
        }
      }
    }
    _ => {
      let mut num = num;
      let mut order: usize = 1000;
      let mut result = String::from("");

      while num >= 1000 {
        let name = orders.get(&order).unwrap();
        let t = match (num / 1000, num % 1000) {
          (upper, 0) => format!("{} {}", encode(upper, ones, tens, orders), name),
          (upper, lower) => {
            format!("{} {} and {}", encode(upper, ones, tens, orders), name, encode(lower, ones, tens, orders))
          }
        };

        result.push_str(&t);
        order *= 1000;
        num /= 1000;
      }

      result
    }
  }
}

fn main() {
  let ones: NumReadableMap = HashMap::from([
    (0, "zero"),
    (1, "one"),
    (2, "two"),
    (3, "three"),
    (4, "four"),
    (5, "five"),
    (6, "six"),
    (7, "seven"),
    (8, "eight"),
    (9, "nine"),
    (10, "ten"),
    (11, "eleven"),
    (12, "twelve"),
    (13, "thirteen"),
    (14, "fourteen"),
    (15, "fifteen"),
    (16, "sixteen"),
    (17, "seventeen"),
    (18, "eighteen"),
    (19, "nineteen"),
  ]);
  
  let tens: NumReadableMap = HashMap::from([
    (0, "zero"), 
    (10, "ten"), 
    (20, "twenty"), 
    (30, "thirty"), 
    (40, "forty"),
    (50, "fifty"), 
    (60, "sixty"), 
    (70, "seventy"), 
    (80, "eighty"), 
    (90, "ninety"),
  ]);
  
  let orders: NumReadableMap = HashMap::from([
    (100, "hundred"),
    (1000, "thousand"),
    (1000000, "million"),
    (1000000000, "billion"),
    (1000000000000, "trillion"),
  ]);

  let mut result: usize = 0;

  for num in 1..=1000 {
    result += encode(num, &ones, &tens, &orders).replace(" ", "").len();
  }

  println!("{}", result);
}
