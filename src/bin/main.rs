use std::error::Error;

use rust_data_structures::heaps::Heap;

fn main() -> Result<(), Box<dyn Error>> {
  let mut heap = Heap::new();

  heap.insert(1);
  heap.insert(2);
  heap.insert(3);
  heap.insert(4);
  heap.insert(5);

  heap.remove()?;
  heap.remove()?;

  println!("{:#?}", heap);

  Ok(())
}