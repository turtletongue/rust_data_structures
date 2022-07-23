use std::error::Error;

use rust_data_structures::heaps::Heap;

fn main() -> Result<(), Box<dyn Error>> {
  let heap = Heap::heapify(vec![1, 6, 2, 5, 3, 4]);

  println!("{:#?}", heap);

  Ok(())
}