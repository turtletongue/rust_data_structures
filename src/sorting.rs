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

pub fn merge_sort<T: PartialOrd + Clone>(array: &mut Vec<T>) {
  let length = array.len();

  if length <= 1 {
    return;
  }

  let middle = array.len() / 2;

  let (mut first_part, mut first_pointer) = (Vec::from(&array[0..middle]), 0);
  merge_sort(&mut first_part);

  let (mut second_part, mut second_pointer) = (Vec::from(&array[middle..length]), 0);
  merge_sort(&mut second_part);

  for index in 0..array.len() {
    let is_first_valid = first_pointer < first_part.len();
    let is_second_valid = second_pointer < second_part.len();

    if !is_first_valid && !is_second_valid {
      break;
    }

    if !is_second_valid || (is_first_valid && first_part[first_pointer] < second_part[second_pointer]) {
      array[index] = first_part[first_pointer].clone();

      first_pointer += 1;
    } else {
      array[index] = second_part[second_pointer].clone();

      second_pointer += 1;
    }
  }
}