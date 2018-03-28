
fn main_1() {
  let end_floor: i64 = include_str!("../data/day1").chars()
  .map(|ch| match ch {
    '(' => 1,
    ')' => -1,
    _ => panic!("Invalid input character: {}", ch),
  })
  .fold(0, |sum, n| sum + n);

  println!("End floor: {}", end_floor);
}

fn main_2() {
  let steps: Vec<i8> = include_str!("../data/day1").chars()
  .map(|ch| match ch {
    '(' => 1,
    ')' => -1,
    _ => panic!("Invalid input character: {}", ch),
  }).collect();

  let mut floor: isize = 0;
  let mut result = 0;
  for (i, &step) in steps.iter().enumerate() {
    floor += step as isize;
    if floor == -1 {
      result = i + 1;
      break;
    }
  }

  println!("Reached -1 at step: {:?}", result);
}

pub fn main () {
  main_1();
  main_2();
}
