pub fn linear_search<T: PartialEq>(array: &Vec<T>, item: T) -> Option<usize> {
  for index in 0..array.len() {
    if array[index] == item {
      return Some(index);
    }
  }

  None
}

pub fn binary_search<T: PartialEq + PartialOrd>(array: &Vec<T>, item: T) -> Option<usize> {
  if array.len() == 0 {
    return None;
  }

  binary_search_recursive(array, item, 0, array.len() - 1)
}

fn binary_search_recursive<T: PartialEq + PartialOrd>(array: &Vec<T>, item: T, left: usize, right: usize) -> Option<usize> {
  let middle = (left + right) / 2;

  if item == array[middle] {
    return Some(middle);
  }

  if left == right {
    return None;
  }

  if item < array[middle] {
    binary_search_recursive(array, item, left, middle - 1)
  } else {
    binary_search_recursive(array, item, middle + 1, right)
  }
}

/*

fn binary_search_iterative<T: PartialEq + PartialOrd>(array: &Vec<T>, item: T) -> Option<usize> {
  let mut left = 0;
  let mut right = array.len() - 1;
  
  while left <= right {
    let middle = (left + right) / 2;

    if item == array[middle] {
      return Some(middle);
    }

    if item < array[middle] {
      right = middle - 1;
    } else {
      left = middle + 1;
    }
  }
  
  None
}

*/

pub fn ternary_search<T: PartialEq + PartialOrd>(array: &Vec<T>, item: T) -> Option<usize> {
  if array.len() == 0 {
    return None;
  }

  ternary_search_recursive(array, item, 0, array.len() - 1)
} 

fn ternary_search_recursive<T: PartialEq + PartialOrd>(array: &Vec<T>, item: T, left: usize, right: usize) -> Option<usize> {
  if left > right {
    return None;
  }

  let partition_size = (right - left) / 3;

  let first_middle = left + partition_size;
  let second_middle = right - partition_size;

  if item == array[first_middle] {
    return Some(first_middle);
  }

  if item == array[second_middle] {
    return Some(second_middle);
  }

  if item < array[first_middle] {
    ternary_search_recursive(array, item, left, first_middle - 1)
  } else if item > array[second_middle] {
    ternary_search_recursive(array, item, second_middle + 1, right)
  } else {
    ternary_search_recursive(array, item, first_middle + 1, second_middle - 1)
  }
}

pub fn jump_search<T: PartialEq + PartialOrd>(array: &Vec<T>, item: T) -> Option<usize> {
  let block_size = (array.len() as f64).sqrt() as usize;

  let mut start = 0;
  let mut next = block_size;

  while item > array[next - 1] && start < array.len() {
    start = next;
    next += block_size;

    if next > array.len() {
      next = array.len();
    }
  }

  for index in start..next {
    if item == array[index] {
      return Some(index);
    }
  }

  None
}