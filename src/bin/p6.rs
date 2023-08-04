// Problem 6: Sum Square Difference

fn square(n: usize) -> usize {
  n * n
}

fn sum_square_difference(n: usize) -> usize {
  square(n * (n + 1) / 2) - (n * (n + 1) * (2 * n + 1) / 6) 
}

fn main() {
  println!("{}", sum_square_difference(100));
}
