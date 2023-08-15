// Problem 27: Number Spiral Diagonals 

fn spiral_sum(grid_size: usize) -> usize {
  let mut stride: usize = 2;
  let mut current = 1;

  let mut numbers: Vec<usize> = vec![];

  while stride <= grid_size {
    numbers.push(current);

    if numbers.len() > 1 && numbers.len() % 4 == 1 {
      stride += 2;
    }
    current += stride;
  }

  numbers.iter().fold(0, |acc, x| acc + x)
}


fn main() {
  println!("{}", spiral_sum(1001));
}
