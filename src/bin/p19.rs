// Problem 19: Counting Sundays

// Uses Zeller's congruence to calculate the weekday of a particular date 
fn which_weekday(date: usize, month: usize, year: usize) -> usize {
  (date + (13 * (month + 1)) / 5 + year + (year / 4) - (year / 100) + (year / 400)) % 7
}

fn main() {
  let mut result = 0;

  for year in 1901..=2000 {
    for month in 1..=12 {
      let weekday: usize;

      if month <= 2 {
        weekday = which_weekday(1, month + 12, year - 1);
      } else { 
        weekday = which_weekday(1, month, year);
      }

      if weekday == 1 {
        result += 1;
      }
    }
  }

  println!("{}", result);
}

