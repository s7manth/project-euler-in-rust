// Problem 15: Lattice Paths 

fn path_ways(size: usize) -> usize {
  let mut grid: Vec<Vec<usize>> = vec![];
  
  for _ in 0..size {
    grid.push(vec![0; size]);
  }

  for x in 0..size {
    grid[0][x] = 1;
    grid[x][0] = 1;
  }

  for i in 1..size {
    for j in 1..size {
      grid[i][j] = grid[i - 1][j] + grid[i][j - 1];
    }
  }

  grid[size - 1][size - 1]
}

fn main() {
  println!("{}", path_ways(21));
}

