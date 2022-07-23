use std::error::Error;

use rust_data_structures::queues::HeapPriorityQueue;

fn main() -> Result<(), Box<dyn Error>> {
  let mut queue = HeapPriorityQueue::new();

  queue.enqueue(4);
  queue.enqueue(8);
  queue.enqueue(1);
  queue.enqueue(1);

  println!("{}", queue.dequeue()?);
  println!("{}", queue.dequeue()?);
  println!("{}", queue.dequeue()?);

  Ok(())
}