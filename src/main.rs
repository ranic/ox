use hash::function::fnv_hash;
use std::io;

mod hash;

fn main() {
  let mut input = String::new();
  println!("Type string to hash:");
  match io::stdin().read_line(&mut input) {
    Ok(n) => {
        println!("{} bytes read", n);
        println!("{}", fnv_hash(&input));
    }
    Err(error) => println!("error: {}", error),
  }
}
