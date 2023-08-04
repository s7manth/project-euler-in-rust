// Problem 2: Even Fibonacci Numbers 

fn even_fib_nums_less_than(limit: u64) -> Vec<u64> {
  let mut result = vec![];

  let mut a: u64 = 1;
  let mut b: u64 = 2;

  while b < limit {
    let temp = b;
    b = a + b;
    a = temp;

    if a % 2 == 0 {
      result.push(a);
    }
  }

  result
}

fn main() {
  let fib_nums: Vec<u64> = even_fib_nums_less_than(4000000);
  let result = fib_nums
    .iter()
    .fold(0, |acc, x| acc + x);

  println!("{}", result);
}
