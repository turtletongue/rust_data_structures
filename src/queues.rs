use std::fmt::{Display, Formatter};

pub trait Queue<T> {
  fn enqueue(&mut self, value: T) -> Result<(), &'static str>;
  fn dequeue(&mut self) -> Result<T, &'static str>;
  fn peek(&self) -> Result<&T, &'static str>;
  fn is_empty(&self) -> bool;
  fn is_full(&self) -> bool {
    false
  }
}

pub struct ArrayQueue<T, const SIZE: usize> {
  items: [Option<T>; SIZE],
  length: usize,
  front: usize,
  back: usize
}

impl <T, const SIZE: usize> ArrayQueue<T, SIZE> {
  const INIT: Option<T> = None;

  pub fn new() -> Self {
    Self {
      items: [ArrayQueue::<T, SIZE>::INIT; SIZE],
      length: 0,
      front: 0,
      back: 0
    }
  }
}

impl<T, const SIZE: usize> Queue<T> for ArrayQueue<T, SIZE> {
  fn enqueue(&mut self, value: T) -> Result<(), &'static str> {
    if self.is_full() {
      return Err("Queue is full");
    }

    if !self.is_empty() {
      self.back = (self.back + 1) % SIZE;
    }
    
    self.items[self.back] = Some(value);
    self.length += 1;

    Ok(())
  }

  fn dequeue(&mut self) -> Result<T, &'static str> {
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

  fn peek(&self) -> Result<&T, &'static str> {
    if self.is_empty() {
      return Err("Queue is empty");
    }

    Ok(self.items[self.front].as_ref().unwrap())
  }

  fn is_empty(&self) -> bool {
    self.length == 0
  }

  fn is_full(&self) -> bool {
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

pub struct PriorityQueue {
  items: Vec<u32>,
}

impl PriorityQueue {
  pub fn new() -> Self {
    Self {
      items: Vec::new()
    }
  }

  fn shift_items_to_insert(&mut self, value: u32) -> usize {
    self.items.push(*self.items.last().unwrap());

    for index in (0..self.items.len()).rev() {
      if index == 0 || self.items[index - 1] < value {
        return index;
      }

      self.items[index] = self.items[index - 1];
    }

    0
  }
}

impl Queue<u32> for PriorityQueue {
  fn enqueue(&mut self, value: u32) -> Result<(), &'static str> {
    if self.is_full() {
      return Err("Queue is full");
    }

    if self.is_empty() {
      self.items.push(value);

      return Ok(());
    }

    let index = self.shift_items_to_insert(value);
    self.items[index] = value;

    Ok(())
  }

  fn dequeue(&mut self) -> Result<u32, &'static str> {
    if self.is_empty() {
      return Err("Queue is empty");
    }

    Ok(self.items.pop().unwrap())
  }

  fn peek(&self) -> Result<&u32, &'static str> {
    if self.is_empty() {
      return Err("Queue is empty");
    }

    Ok(self.items.first().unwrap())
  }

  fn is_empty(&self) -> bool {
    self.items.len() == 0
  }
}

impl Display for PriorityQueue {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let mut formatted_items = String::new();

    for (index, item) in self.items.iter().enumerate() {
      let delimeter = if index != self.items.len() - 1 { " <- " } else { "" };

      formatted_items.push_str(&(item.to_string() + delimeter));
    }

    write!(f, "[{}]", formatted_items)?;

    Ok(())
  }
}