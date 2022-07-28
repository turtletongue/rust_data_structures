use rust_data_structures::sorting;

fn main() {
  let mut array = vec![5, 7, 3, 1, 2];

  sorting::insertion_sort(&mut array);

  println!("{:#?}", array);
}