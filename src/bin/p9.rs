// Problem 9: Special Pythagorean Triplet 

fn pyth_triplet_product(n: usize) -> usize {
  let mut c = n / 3 + 1;
  let mut result = 0;

  while c < n / 2 {
    if c * c + 2 * n * c < n * n {
      c += 1;
      continue;
    }

    let a_b_sq = c.pow(2) + 2 * n * c - n.pow(2);
    let a_b = (a_b_sq as f64).sqrt() as usize;

    if a_b.pow(2) == a_b_sq {
      let a = (n - c + a_b) / 2;
      let b = n - c - a;
      result = a * b * c;
      break;
    }

    c += 1;
  }

  result
}

fn main() {
  println!("{}", pyth_triplet_product(1000));
}
