#[derive(PartialEq)]
enum HeapChild {
  Left,
  Right,
}

#[derive(Debug)]
pub struct Heap<T> {
  items: Vec<T>,
}

impl Heap<i32> {
  pub fn new() -> Self {
    Self { items: Vec::new() }
  }

  pub fn heapify(items: Vec<i32>) -> Self {
    let length = items.len();

    let mut heap = Self { items };

    for index in 1..length {
      heap.bubble(index);
    }

    heap
  }

  pub fn insert(&mut self, value: i32) {
    self.items.push(value);

    self.bubble(self.items.len() - 1);
  }

  pub fn remove(&mut self) -> Result<i32, &'static str> {
    if self.is_empty() {
      return Err("Heap is empty");
    }

    let last_child = self.items.pop().unwrap();

    if self.is_empty() {
      return Ok(last_child);
    }

    let first = self.items[0];

    self.items[0] = last_child;
    self.dive(0);

    Ok(first)
  }

  pub fn is_empty(&self) -> bool {
    self.items.len() == 0
  }

  fn bubble(&mut self, index: usize) {
    if index == 0 {
      return;
    }

    let parent_index = Self::get_parent_index(index);

    if self.items[parent_index] >= self.items[index] {
      return;
    }

    self.items.swap(parent_index, index);

    self.bubble(parent_index);
  }

  fn dive(&mut self, index: usize) {
    if index >= self.items.len() {
      return;
    }

    let swap_index = self.get_dive_swap_index(index);

    if swap_index.is_none() {
      return;
    }

    let swap_index = swap_index.unwrap();

    self.items.swap(swap_index, index);

    self.dive(swap_index);
  }

  fn get_dive_swap_index(&self, index: usize) -> Option<usize> {
    let (left_child, left_child_index) = self.left_child(index);
    let (right_child, right_child_index) = self.right_child(index);
    let value = self.items[index];

    let mut swap_index = index;

    if left_child.is_some() && *left_child.unwrap() > value {
      swap_index = left_child_index;
    }

    if right_child.is_some() && *right_child.unwrap() > value {
      swap_index = right_child_index;
    }

    if swap_index == index {
      return None;
    }

    Some(swap_index)
  }

  fn left_child(&self, index: usize) -> (Option<&i32>, usize) {
    let left_child_index = Self::get_child_index(index, HeapChild::Left);

    (self.items.get(left_child_index), left_child_index)
  }

  fn right_child(&self, index: usize) -> (Option<&i32>, usize) {
    let right_child_index = Self::get_child_index(index, HeapChild::Right);

    (self.items.get(right_child_index), right_child_index)
  }

  fn get_child_index(parent_index: usize, child_type: HeapChild) -> usize {
    parent_index * 2 + if child_type == HeapChild::Left { 1 } else { 2 }
  }

  fn get_parent_index(child_index: usize) -> usize {
    (child_index - 1) / 2
  }
}