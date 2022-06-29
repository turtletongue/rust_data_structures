use std::error::Error;

use rust_data_structures::lists::LinkedList;

fn main() -> Result<(), Box<dyn Error>> {
  let mut list = LinkedList::new();

  list.add_last(1);
  list.add_last(2);
  list.add_last(3);

  list.remove_first()?;
  list.remove_last()?;

  println!("{list}");

  Ok(())
}
