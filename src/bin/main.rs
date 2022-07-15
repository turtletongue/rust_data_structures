use std::error::Error;

use rust_data_structures::queues::{Queue, PriorityQueue};

fn main() -> Result<(), Box<dyn Error>> {
  let mut queue = PriorityQueue::new();

  queue.enqueue(5)?;
  queue.enqueue(1)?;

  println!("{queue}");

  queue.dequeue()?;
  queue.dequeue()?;

  queue.enqueue(7)?;
  queue.enqueue(5)?;
  queue.enqueue(8)?;
  queue.enqueue(1)?;
  queue.enqueue(1)?;
  queue.enqueue(6)?;

  println!("{queue}");

  println!("{}", queue.peek()?);

  Ok(())
}