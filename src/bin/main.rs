use rust_data_structures::arrays::DynamicArray;

fn main() {
  let mut array = DynamicArray::new(3);

  array.insert(1);
  array.insert(2);
  array.insert(3);

  array.remove_at(2);
  
  array.insert(7);

  println!("{array}");
}
