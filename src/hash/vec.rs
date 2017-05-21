/*
  Source: hash/vec.rs
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

use hash::functions;

enum HashEntry<T> {
  Value {key: String, data: T, hash: u64, index: usize},
  Tombstone {index: usize},
  Empty,
}

pub struct Hash<T> {
  capacity: usize,
  table: Vec<HashEntry<T>>,
}

impl <T> Hash<T> {
  pub fn new(buckets: usize) -> Hash<T> {
    let mut hash = Hash {capacity: buckets, table: Vec::with_capacity(buckets)};
    for _ in 0..buckets {
      hash.table.push(HashEntry::Empty);
    }
    return hash;
  }

  pub fn get(&self, key: String) -> Option<&T> {
    let hash = hash_func(&key);
    let initial_i = self.hash_to_index(hash);
    for offset in 0..self.capacity {
      let i = (initial_i + offset) % self.capacity;
      match self.table[i] {
        HashEntry::Empty => {return None;}
        HashEntry::Tombstone {..} => (),
        HashEntry::Value {key: ref target_key, ref data, hash: ref target_hash, ..} => {
          if hash == *target_hash && key == *target_key {
            return Some(data);
          }
        },
      };
    }
    return None;
  }

  pub fn set(&mut self, key: String, value: T) {
    let hash = hash_func(&key);
    let initial_i = self.hash_to_index(hash);
    match self.find_set_index(&key, initial_i) {
      Some(i) => {self.table[i] = HashEntry::Value {key, data: value, hash, index: i}; return;}
      None => panic!("RESIZE ME!")
    }
  }

  pub fn del(&mut self, key: String) {
    let hash = hash_func(&key);
    let initial_i = self.hash_to_index(hash);
    match self.find_del_index(&key, initial_i) {
      Some((index, initial_index)) => {self.table[index] = HashEntry::Tombstone {index: initial_index};},
      None => (),
    }
  }

  fn hash_to_index(&self, hash: u64) -> usize{
    return (hash % self.capacity as u64) as usize;
  }

  fn find_del_index(&mut self, key: &String, initial_i: usize) -> Option<(usize, usize)> {
    for offset in 0..self.capacity {
      let i = (initial_i + offset) % self.capacity;

      match self.table[i] {
        HashEntry::Value {key: ref target_key, index, ..} => {
          if *key == *target_key {
            return Some((i, index));
          }
        },
        HashEntry::Empty => {break;},
        _ => (),
      };
    }
    return None;
  }

  fn find_set_index(&mut self, key: &String, initial_i: usize) -> Option<usize> {
    for offset in 0..self.capacity {
      let i = (initial_i + offset) % self.capacity;

      match self.table[i] {
        HashEntry::Empty => {return Some(i)},
        HashEntry::Tombstone {index: target_index} => {
          if i == target_index {
            return Some(i);
          }
        },
        HashEntry::Value {key: ref target_key, ..} => {
          if *key == *target_key {
            return Some(i);
          }
        }
      };
    }
    return None;
  }
}

fn hash_func(key: &String) -> u64 {
  return functions::fnv_hash(key);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_from_hash() {
    let mut hash = Hash::new(5);

    let k = String::from("foo");
    let v = String::from("bar");

    hash.set(k, v);

    assert_eq!(hash.get(String::from("foo")), Some(&String::from("bar")));
  }

  #[test]
  fn test_get_non_existant_from_hash() {
    let mut hash = Hash::new(5);
    let k = String::from("foo");
    let v = String::from("bar");

    hash.set(k, v);

    assert_eq!(hash.get(String::from("bar")), None);
  }

  #[test]
  fn test_override() {
    let mut hash = Hash::new(5);

    let k1 = String::from("foo");
    let v1 = String::from("bar");
    let k2 = String::from("foo");
    let v2 = String::from("baz");

    hash.set(k1, v1);
    hash.set(k2, v2);

    assert_eq!(hash.get(String::from("foo")), Some(&String::from("baz")));
  }

  #[test]
  fn test_delete() {
    let mut hash = Hash::new(5);

    let k1 = String::from("foo");
    let v1 = String::from("bar");
    let k2 = String::from("baz");
    let v2 = String::from("bar");

    hash.set(k1, v1);
    hash.set(k2, v2);
    hash.del(String::from("foo"));

    assert_eq!(hash.get(String::from("foo")), None);
    assert_eq!(hash.get(String::from("baz")), Some(&String::from("bar")));
  }

  #[test]
  fn test_delete_and_set() {
    let mut hash = Hash::new(5);

    let k1 = String::from("foo");
    let v1 = String::from("bar");
    let k2 = String::from("baz");
    let v2 = String::from("bar");
    let k3 = String::from("foo");
    let v3 = String::from("foobar");

    hash.set(k1, v1);
    hash.set(k2, v2);
    hash.del(String::from("foo"));
    hash.set(k3, v3);

    assert_eq!(hash.get(String::from("foo")), Some(&String::from("foobar")));
    assert_eq!(hash.get(String::from("baz")), Some(&String::from("bar")));
  }
}
