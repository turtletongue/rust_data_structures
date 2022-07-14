use crate::lists::LinkedList;

pub struct Stack<T: Clone> {
  items: LinkedList<T>
}

impl<T: Clone + PartialEq> Stack<T> {
  pub fn new() -> Self {
    Self {
      items: LinkedList::<T>::new()
    }
  }

  pub fn push(&mut self, value: T) {
    self.items.add_first(value);
  }

  pub fn peek(&self) -> Result<T, &str> {
    if self.is_empty() {
      return Err("Stack is empty");
    }
    self.items.get_kth_from_end(self.items.size())
  }

  pub fn pop(&mut self) -> Result<T, &str> {
    if self.is_empty() {
      return Err("Stack is empty");
    }

    let item_to_pop = self.items.get_kth_from_end(self.items.size())?;

    self.items.remove_first()?;

    Ok(item_to_pop)
  }

  pub fn is_empty(&self) -> bool {
    self.items.is_empty()
  }
}