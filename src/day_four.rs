use crypto::digest::Digest;
use crypto::md5::Md5;

fn candidate_has_zeroes(candidate: String, zeroes: usize) -> bool {
  let slice = &candidate[0..zeroes];
  for ch in slice.chars() {
    if ch != '0' {
      return false
    }
  }
  return true;
}

fn hash_candidate(prefix: &str, suffix: usize) -> String {
  let candidate = format!("{}{}", prefix, suffix);
  let mut hasher = Md5::new();
  hasher.input_str(&candidate);
  hasher.result_str()
}

fn find_suffix_with_zeroes(prefix: &str, n_zeroes: usize) -> usize {
  let mut suffix = 0;
  let mut candidate = hash_candidate(prefix, suffix);
  while !candidate_has_zeroes(candidate, n_zeroes) {
    suffix += 1;
    candidate = hash_candidate(prefix, suffix);
  }
  suffix
}

fn part_1(prefix: &str) {
  let suffix = find_suffix_with_zeroes(prefix, 5);
  println!("First suffix with 5 zeros = {:?}", suffix);
}

fn part_2(prefix: &str) {
  let suffix = find_suffix_with_zeroes(prefix, 6);
  println!("First suffix with 6 zeros = {:?}", suffix);
}

pub fn main() {
  let prefix = "ckczppom";
  part_1(&prefix);
  part_2(&prefix);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn find_with_5_zeroes_works() {
    assert_eq!(609043, find_suffix_with_zeroes("abcdef", 5));
    assert_eq!(1048970, find_suffix_with_zeroes("pqrstuv", 5));
    assert_eq!(117946, find_suffix_with_zeroes("ckczppom", 5));
  }

  #[test]
  fn find_with_6_zeroes_works() {
    assert_eq!(3938038, find_suffix_with_zeroes("ckczppom", 6));
  }
}
