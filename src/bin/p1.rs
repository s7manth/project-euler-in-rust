// Problem 1: Multiples of 3 or 5

fn main() {
  let mut result: u64 = 0;
  
  for i in 1..1000 {
    if i % 3 == 0 || i % 5 == 0 {
      result += i;
    }
  }

  println!("Result: {result}");
}

