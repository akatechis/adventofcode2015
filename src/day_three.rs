use std::collections::HashMap;
use self::data::*;
use self::data::Direction::*;

type Address = (isize, isize);

fn move_in_direction((x, y): Address, dir: &Direction) -> Address {
  match *dir {
    N => (x, y + 1),
    S => (x, y - 1),
    E => (x + 1, y),
    W => (x - 1, y),
  }
}

fn count_presents_delivered(directions: &Vec<Direction>) -> HashMap<Address, usize> {
  let mut presents: HashMap<Address, usize> = HashMap::new();
  let mut address: Address = (0, 0);

  // deliver one present at the initial location
  presents.entry(address).or_insert(1);

  for dir in directions {
    address = move_in_direction(address, dir);

    let entry = presents.entry(address).or_insert(0);
    *entry += 1;
  }

  presents
}

fn merge_present_maps(a: &HashMap<Address, usize>, b: &HashMap<Address, usize>) -> HashMap<Address, usize> {
  let mut total = a.clone();

  for (addr, presents) in b {
    let entry = total.entry(*addr).or_insert(0);
    *entry += presents;
  }

  total
}

fn part_1(directions: &Vec<Direction>) {
  let presents = count_presents_delivered(&directions);

  println!("Number of homes that got a present: {}", presents.len());
}

fn part_2(directions: Vec<Direction>) {
  let mut santa_dir = Vec::with_capacity(directions.len() / 2 + 1);
  let mut robo_dir = Vec::with_capacity(directions.len() / 2 + 1);
  let mut i = 0;

  for dir in directions {
    if i % 2 == 0 {
      santa_dir.push(dir);
    }
    else {
      robo_dir.push(dir);
    }
    i += 1;
  }

  let santa_presents = count_presents_delivered(&santa_dir);
  let robo_presents = count_presents_delivered(&robo_dir);

  let total_presents = merge_present_maps(&santa_presents, &robo_presents);

  println!("Total houses visited: {:?}", total_presents.len());
}

pub fn main () {
  let directions = data::directions();
  part_1(&directions);
  part_2(directions);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn count_presents_delivered_empty_works() {
    let directions = vec![];
    let presents = count_presents_delivered(&directions);

    assert_eq!(1, presents.len());
    assert_eq!(1, *presents.get(&(0,0)).unwrap());
  }

  #[test]
  fn count_presents_delivered_one_works() {
    let directions = vec![N];
    let presents = count_presents_delivered(&directions);

    assert_eq!(2, presents.len());
    assert_eq!(1, *presents.get(&(0,0)).unwrap());
    assert_eq!(1, *presents.get(&(0,1)).unwrap());
  }

  #[test]
  fn count_presents_delivered_multiple_unique_works() {
    let directions = vec![N, E, S, W];
    let presents = count_presents_delivered(&directions);

    assert_eq!(4, presents.len());
    assert_eq!(2, *presents.get(&(0,0)).unwrap());
    assert_eq!(1, *presents.get(&(0,1)).unwrap());
    assert_eq!(1, *presents.get(&(1,1)).unwrap());
    assert_eq!(1, *presents.get(&(1,0)).unwrap());
  }

  #[test]
  fn count_presents_delivered_multiple_repeated_works() {
    let directions = vec![N, S, N, S, N, S, N, S, N, S, ];
    let presents = count_presents_delivered(&directions);

    assert_eq!(2, presents.len());
    assert_eq!(6, *presents.get(&(0,0)).unwrap());
    assert_eq!(5, *presents.get(&(0,1)).unwrap());
  }

  #[test]
  fn move_in_direction_works() {
    let addr = (0, 0);
    assert_eq!((0, 1), move_in_direction(addr, &N));
    assert_eq!((0, -1), move_in_direction(addr, &S));
    assert_eq!((1, 0), move_in_direction(addr, &E));
    assert_eq!((-1, 0), move_in_direction(addr, &W));
  }
}

mod data {

  #[derive(Debug)]
  pub enum Direction {
    N, S, E, W
  }

  fn input() -> &'static str {
    include_str!("../data/day3")
  }

  pub fn directions() -> Vec<Direction> {
    use self::Direction::*;

    input().chars()
    .map(|ch| match ch {
      '^' => N, '>' => E, 'v' => S, '<' => W,
      _ => panic!("Invalid input character: {}", ch),
    })
    .collect()
  }
}
