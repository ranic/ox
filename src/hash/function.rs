pub fn fnv_hash(s: &str) -> u64
{
  let mut h: u64 = 2166136261;

  for c in s.chars() {
    let i = c as u64;
    h = h.wrapping_mul(16777619) ^ i;
  }
  return h;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_fnv_hash() {
      assert_eq!(18098019522363481619, fnv_hash("foo"));
  }
}