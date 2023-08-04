// Problem 7: 10001st Prime

fn is_prime(n: usize) -> bool {
  let sqrt = (n as f64).sqrt() as usize;

  for i in 2..(sqrt + 1) {
    if n % i == 0 {
      return false;
    }
  }

  true
}

fn main() {
  let mut counter = 2;
  let mut current = 3;

  while counter <= 10001 {
    while !is_prime(current) {
      current += 2;
    }

    println!("{} {}", counter, current);

    current += 2;
    counter += 1;
  }
}
