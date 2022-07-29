use rust_data_structures::sorting;

fn main() {
  let mut array = vec![7, 3, 5, 2, 3, 1, 5, 8];

  sorting::quick_sort(&mut array);

  println!("{:#?}", array);
}