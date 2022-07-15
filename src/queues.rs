use std::fmt::{Display, Formatter};

pub struct ArrayQueue<T, const SIZE: usize> {
  items: [Option<T>; SIZE],
  length: usize,
  front: usize,
  back: usize
}

impl<T, const SIZE: usize> ArrayQueue<T, SIZE> {
  const INIT: Option<T> = None;

  pub fn new() -> Self {
    Self {
      items: [ArrayQueue::<T, SIZE>::INIT; SIZE],
      length: 0,
      front: 0,
      back: 0
    }
  }

  pub fn enqueue(&mut self, value: T) -> Result<(), &'static str> {
    if self.is_full() {
      return Err("Queue is full")
    }

    if !self.is_empty() {
      self.back = (self.back + 1) % SIZE;
    }
    
    self.items[self.back] = Some(value);
    self.length += 1;

    Ok(())
  }

  pub fn dequeue(&mut self) -> Result<T, &'static str> {
    if self.is_empty() {
      return Err("Queue is empty");
    }

    let value = self.items[self.front].take().unwrap();

    if self.length != 1 {
      self.front = (self.front + 1) % SIZE;
    }
    
    self.length -= 1;
    
    Ok(value)
  }

  pub fn peek(&self) -> Result<&T, &'static str> {
    if self.is_empty() {
      return Err("Queue is empty");
    }

    Ok(self.items[self.front].as_ref().unwrap())
  }

  pub fn is_empty(&self) -> bool {
    self.length == 0
  }

  pub fn is_full(&self) -> bool {
    self.length == SIZE
  }
}

impl<T: Display, const SIZE: usize> Display for ArrayQueue<T, SIZE> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let mut formatted_items = String::new();

    let mut current_index = self.front;
    let mut seen_count: usize = 0;

    while seen_count < self.length {
      seen_count += 1;

      let delimeter = if seen_count != self.length { " <- " } else { "" };

      formatted_items.push_str(&(self.items[current_index].as_ref().unwrap().to_string() + delimeter));

      current_index += 1;

      if current_index == SIZE {
        current_index = 0;
      }
    }

    write!(f, "[{}]", formatted_items)?;

    Ok(())
  }
}