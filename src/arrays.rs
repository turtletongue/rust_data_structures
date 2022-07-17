use std::alloc::{Layout, alloc, dealloc};
use std::fmt::Display;

type ArrayItem = u8;

const NOT_FOUND_INDEX: isize = -1;
const ARRAY_QUARTER: usize = 4;

pub struct DynamicArray {
  items: *mut ArrayItem,
  layout: Layout,
  length: usize,
  capacity: usize
}

impl DynamicArray {
  pub fn new(size: usize) -> Self {
    unsafe {
      let (items, layout) = Self::allocate_unsafe(size);

      Self {
        items,
        layout,
        length: 0,
        capacity: size,
      }
    }
  }

  pub fn insert(&mut self, item: ArrayItem) {
    if self.length < self.capacity {
      unsafe {
        self.insert_unsafe(self.length, item);
      }

      self.length += 1;

      return;
    }

    unsafe {
      self.copy_with_allocation_unsafe(if self.length == 0 { 1 } else { self.length * 2 });

      self.insert_unsafe(self.length, item);
    }

    self.length += 1;
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

    for i in index..self.length {
      unsafe {
        self.insert_unsafe(i, *self.items.add(i + 1));
      }
    }

    let should_shrink = self.length != 1 && self.capacity / (self.length - 1) >= ARRAY_QUARTER;

    if should_shrink {
      unsafe {
        self.copy_with_allocation_unsafe(self.capacity - (self.capacity / 2));
      }
    }

    self.length -= 1;
  }

  unsafe fn insert_unsafe(&mut self, index: usize, item: ArrayItem) {
    *self.items.add(index) = item;
  }

  unsafe fn remove_at_unsafe(&mut self, index: usize) {
    *self.items.add(index) = ArrayItem::default();
  }

  unsafe fn copy_with_allocation_unsafe(&mut self, size: usize) {
    let initial_items = self.items;
    let initial_layout = self.layout;

    (self.items, self.layout) = Self::allocate_unsafe(size);

    self.capacity = size;

    for index in 0..self.length {
      self.insert_unsafe(index, *initial_items.add(index));
    }

    dealloc(initial_items, initial_layout);
  }

  unsafe fn allocate_unsafe(size: usize) -> (*mut ArrayItem, Layout) {
    let layout = Layout::array::<ArrayItem>(size).unwrap();

    (alloc(layout) as *mut ArrayItem, layout)
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