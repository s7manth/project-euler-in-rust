// Problem 5: Smallest Multiple

fn smallest_multiple(limit: usize) -> usize {
  let mut result = 1;
  let mut considered_nums: Vec<usize> = vec![];

  for i in 2..(limit + 1) {
    if result % i != 0 {
      let mut temp = i;
      for n in &considered_nums {
        if temp % n == 0 {
          temp /= n;
        }
      }
      println!("{} {}", result, temp);
      considered_nums.push(temp);
      result *= temp;
    }
  }

  result
}

fn main() {
  println!("{}", smallest_multiple(20));
}
