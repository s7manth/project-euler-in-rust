// Problem 24: Lexicographic Permutations 

// The main idea is that you repeatedly compute what is left off the rank. The number is 
// obtained by appending what is left in the array based on the index calculation that is achieved 
// upon factorial based reduction. 
fn main() {
  let mut factorials: Vec<usize> = vec![1];
  let mut current = 1;
  for num in 1..=9 {
    current *= num;
    factorials.push(current);
  }

  let mut numbers_to_work_with: Vec<usize> = (0..=9).collect();

  let mut rank = 999999;
  let mut result: Vec<usize> = vec![];
  for &f in factorials.iter().rev() {
    let index = rank / f;
    result.push(numbers_to_work_with[index]);
    numbers_to_work_with.remove(index);
    rank %= f;
  }

  println!("{:?}", result);
}
