
fn md5_i(x: u32, y: u32, z: u32) -> u32 {
  y ^ (x | !z)
}

fn md5_h(x: u32, y: u32, z: u32) -> u32 {
  x ^ y ^ z
}

fn md5_g(x: u32, y: u32, z: u32) -> u32 {
  (x & z) | (y & !z)
}

fn md5_f(x: u32, y: u32, z: u32) -> u32 {
  (x & y) | (!x & z)
}

fn md5_digest(input: &[u8]) -> &[u8] {
  let mut md_buf: [u32; 4] = [0x01234567, 0x89abcdef, 0xfedcba98, 0x76543210];
}

fn part_1() {

}

pub fn main() {
  part_1();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn md5_digest_works() {
    let hash = md5_digest("rust".as_bytes());
    assert_eq!("72812e30873455dcee2ce2d1ee26e4ab".as_bytes(), hash);
  }

  #[test]
  fn md5_f_works() {
    assert_eq!(23280, md5_f(0xA41F, 0x0010, 0xFEED));
  }

  #[test]
  fn md5_g_works() {
    assert_eq!(42013, md5_g(0xA41F, 0x0010, 0xFEED));
  }

  #[test]
  fn md5_h_works() {
    assert_eq!(23266, md5_h(0xA41F, 0x0010, 0xFEED));
  }

  #[test]
  fn md5_i_works() {
    assert_eq!(4294944015, md5_h(0xA41F, 0x0010, 0xFEED));
  }
}
