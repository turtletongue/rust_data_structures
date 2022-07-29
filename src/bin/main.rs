use rust_data_structures::sorting;

fn main() {
  let mut array = vec![7, 3, 5, 2, 3, 1, 5, 8];

  sorting::bucket_sort(&mut array, 3);

  println!("{:#?}", array);
}