// Problem 16: Power Digit Sum 

fn power_digit_sum(number: usize, exponent: usize) -> usize {
  if exponent == 0 {
    return 1;
  }

  let mut arr = vec![];
  arr.push(1);

  for _ in 0..exponent {
    let mut carry: usize = 0;

    for d in arr.iter_mut() {
      let product = *d * number + carry;
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
  println!("{}", power_digit_sum(2, 1000));
}
