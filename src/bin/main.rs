use std::error::Error;

use rust_data_structures::queues::ArrayQueue;

fn main() -> Result<(), Box<dyn Error>> {
  let mut queue = ArrayQueue::<i32, 5>::new();

  queue.enqueue(1)?;
  queue.enqueue(5)?;

  println!("{queue}");

  queue.dequeue()?;
  queue.dequeue()?;

  queue.enqueue(5)?;
  queue.enqueue(6)?;
  queue.enqueue(7)?;

  println!("{queue}");

  println!("{}", queue.peek()?);

  Ok(())
}