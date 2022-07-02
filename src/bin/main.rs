use std::error::Error;

use rust_data_structures::lists::LinkedList;

fn main() -> Result<(), Box<dyn Error>> {
  let mut list = LinkedList::new();

  list.add_last(1);
  list.add_last(4);
  list.add_last(5);

  println!("{}", list.get_kth_from_end(1)?);

  Ok(())
}
