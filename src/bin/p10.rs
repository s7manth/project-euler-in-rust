// Problem 10: Summation of Primes 

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
  let mut result = 0;
  for num in 2..2000000 {
    if is_prime(num) {
      result += num;
    }
  }

  println!("{}", result);
}
