use rust_data_structures::sorting;

fn main() {
  let mut array = vec![7, 3, 5, 2, 3, 1, 5, 8];

  sorting::counting_sort(&mut array, 8);

  println!("{:#?}", array);
}