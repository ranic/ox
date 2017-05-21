/*
  Source: hash/functions.rs
  Copyright (C) 2017 Akshay Nanavati <https://github.com/akshaynanavati>

  This program is free software: you can redistribute it and/or modify
  it under the terms of the GNU General Public License as published by
  the Free Software Foundation, either version 3 of the License, or
  (at your option) any later version.

  This program is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  GNU General Public License for more details.

  You should have received a copy of the GNU General Public License
  along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

pub fn fnv_hash(s: &String) -> u64
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
      assert_eq!(18098019522363481619, fnv_hash(&String::from("foo")));
  }
}
