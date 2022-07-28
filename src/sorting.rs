pub fn bubble_sort<T: PartialOrd>(array: &mut Vec<T>) {
  for i in 0..array.len() {
    let mut is_sorted = true;

    for index in 1..(array.len() - i) {
      if array[index - 1] > array[index] {
        is_sorted = false;

        array.swap(index - 1, index);
      }
    }

    if is_sorted {
      break;
    }
  }
}

pub fn selection_sort<T: PartialOrd>(array: &mut Vec<T>) {
  for i in 0..array.len() {
    let mut min_index = i;

    for index in i..array.len() {
      if array[index] < array[min_index] {
        min_index = index;
      }
    }

    array.swap(i, min_index);
  }
}

pub fn insertion_sort<T: PartialOrd + Clone>(array: &mut Vec<T>) {
  for index in 1..array.len() {
    let current = &array[index].clone();

    let mut sorted_index = index;

    while sorted_index > 0 && array[sorted_index - 1] > *current {
      array[sorted_index] = array[sorted_index - 1].clone();

      sorted_index -= 1;
    }

    array[sorted_index] = current.clone();    
  }
}