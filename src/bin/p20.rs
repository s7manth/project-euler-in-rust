// Problem 16: Factorial Digit Sum

fn factorial_digit_sum(number: usize) -> usize {
  if number == 0 {
    return 1;
  }

  let mut arr = vec![];
  arr.push(1);

  for i in 0..number {
    let mut carry: usize = 0;

    for d in arr.iter_mut() {
      let product = *d * (number - i) + carry;
      *d = product % 10;
      carry = product / 10;
    }

    while carry != 0 {
      arr.push(carry % 10);
      carry /= 10;
    }
  }

  arr
    .iter()
    .fold(0, |acc, x| acc + x)
}

fn main() {
  println!("{}", factorial_digit_sum(100));
}
