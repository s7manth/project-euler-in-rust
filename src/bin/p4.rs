// Problem 4: Largest Palindrome Product 

use std::time::Instant;

fn is_palindrome(num: u64) -> bool {
  let reversed_string: String = num
    .to_string()
    .chars()
    .rev()
    .collect::<String>();

  let reversed_num: u64 = reversed_string
    .parse()
    .unwrap();
  
  reversed_num ^ num == 0
}

fn palindrome_product(num_digits: usize) -> u64 {
  let limit: u64 = String::from("9")
    .repeat(num_digits)
    .parse()
    .expect("Not a number!");

  for i in ((limit - (limit / 10))..(limit + 1)).rev() {
    for j in ((limit - (limit / 10))..(limit + 1)).rev() {
      let product = i * j;
      if product % 11 != 0 {
        continue;
      }

      if is_palindrome(product) {
        println!("{} {}", i, j);
        return product;
      }
    }
  }

  return 0;
}

fn main() {
  let now = Instant::now();

  {
    println!("{}", palindrome_product(3));
  }

  let elapsed = now.elapsed();
  println!("Elapsed: {:.2?}", elapsed);
}

