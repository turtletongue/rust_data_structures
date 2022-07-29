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