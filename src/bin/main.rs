use std::error::Error;

use rust_data_structures::stacks::Stack;

fn main() -> Result<(), Box<dyn Error>> {
  let mut stack = Stack::new();

  stack.push(1);
  stack.push(2);
  stack.push(3);

  stack.pop()?;

  println!("{}", stack.pop()?);
  stack.push(5);
  println!("{}", stack.peek()?);

  println!("{}", stack.is_empty());

  Ok(())
}