pub mod function;

struct Hash {
  capacity: i32,
  table: Vec<Vec<u8>>,
}