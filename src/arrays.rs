use std::alloc::{Layout, alloc};
use std::fmt::Display;

type ArrayItem = u8;

const NOT_FOUND_INDEX: isize = -1;
const ARRAY_QUARTER: usize = 4;

pub struct DynamicArray {
  items: *mut ArrayItem,
  length: usize,
  capacity: usize
}

impl DynamicArray {
  pub fn new(size: usize) -> Self {
    unsafe {
      Self {
        items: Self::allocate_unsafe(size),
        length: 0,
        capacity: size,
      }
    }
  }

  pub fn insert(&mut self, item: ArrayItem) {
    let initial_length = self.length;
    self.length += 1;

    if initial_length < self.capacity {
      unsafe {
        self.insert_unsafe(initial_length, item);
      }

      return;
    }

    let initial_items = self.items;

    self.capacity = if initial_length == 0 { 1 } else { initial_length * 2 };

    unsafe {
      self.items = Self::allocate_unsafe(self.capacity);
    }

    unsafe {
      for index in 0..initial_length {
        self.insert_unsafe(index, *initial_items.add(index));
      }

      self.insert_unsafe(initial_length, item);
    }

  }

  pub fn index_of(&self, item: ArrayItem) -> isize {
    for index in 0..self.length {
      unsafe {
        if item == *self.items.add(index) {
          return index as isize;
        }
      }
    }

    NOT_FOUND_INDEX
  }

  pub fn remove_at(&mut self, index: usize) {
    assert!(index < self.length, "Array out of bounds");

    unsafe {
      self.remove_at_unsafe(index);
    }

    self.length -= 1;

    for i in index..self.length {
      println!("{} become {i}", i + 1);
      unsafe {
        self.insert_unsafe(i, *self.items.add(i + 1));
      }
    }

    let should_shrink = self.length != 0 && self.capacity / self.length >= ARRAY_QUARTER;

    if should_shrink {
      let initial_items = self.items;

      unsafe {
        self.items = Self::allocate_unsafe(self.length);
      }

      let mut allocated_index = 0;

      for i in 0..self.length {
        unsafe {
          self.insert_unsafe(allocated_index, *initial_items.add(i));
        }
  
        allocated_index += 1;
      }

      self.capacity = self.length;
    }
  }

  unsafe fn insert_unsafe(&mut self, index: usize, item: ArrayItem) {
    *self.items.add(index) = item;
  }

  unsafe fn remove_at_unsafe(&mut self, index: usize) {
    *self.items.add(index) = ArrayItem::default();
  }

  unsafe fn allocate_unsafe(size: usize) -> *mut ArrayItem {
    let layout = Layout::array::<ArrayItem>(size).unwrap();

    alloc(layout) as *mut ArrayItem
  }
}

impl Display for DynamicArray {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    unsafe {
      let mut formatted_items = String::new();

      for index in 0..self.length {
        let item = (*self.items.add(index)).to_string();

        let delimeter = if index == self.length - 1 { "" } else { ", " };

        formatted_items.push_str(&(item + delimeter));
      }

      write!(f, "[{}]", formatted_items)?;
    }

    Ok(())
  }
}