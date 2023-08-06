// Problem 25: 1000-digit Fibonacci Number 

fn fibonacci_index_by_digits(digits: usize) -> usize {
  let mut a: Vec<usize> = vec![];
  let mut b: Vec<usize> = vec![];
  
  a.push(1);
  b.push(1);

  let mut index = 2;
  while b.len() < digits {
    let mut carry = 0;
    for i in 0..std::cmp::min(a.len(), b.len()) {
      let temp = b[i];
      carry += a[i] + b[i];
      b[i] = carry % 10;
      a[i] = temp;
      carry /= 10;
    }

    for j in std::cmp::min(a.len(), b.len())..b.len() {
      a.push(b[j]);
      carry += b[j];
      b[j] = carry % 10;
      carry /= 10;
    }

    while carry != 0 {
      b.push(carry % 10);
      carry /= 10;
    }

    index += 1;
  }

  index
}

fn main() {
  println!("{}", fibonacci_index_by_digits(1000));
}
