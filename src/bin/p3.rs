// Problem 3: Largest Prime Factor 

fn prime_factors(mut num: u64) -> Vec<u64> {
  let mut result = vec![];
  let sqrt = (num as f64).sqrt() as u64;

  for i in 2..sqrt {
    if num % i == 0 {
      result.push(i);
    }

    while num != 0 && num % i == 0 {
      num /= i;
    }
  }

  result
}

fn main() {
  let pfs = prime_factors(600851475143);

  println!("{:?}", pfs);
}
