use std::error::Error;
use std::time::{Instant};

use rust_data_structures::lists::LinkedList;

fn main() -> Result<(), Box<dyn Error>> {
  let mut list = LinkedList::new();

  for i in 0..100_000_000 {
    list.add_last(i);
  }

  println!("created");

  let now = Instant::now();

  list.reverse();

  println!("reversed in {}s", now.elapsed().as_secs());

  Ok(())
}
