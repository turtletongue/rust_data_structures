use rust_data_structures::search;

fn main() {
  let array = vec![1, 2, 3, 4, 5, 6];

  println!("{}", search::ternary_search(&array, 3).unwrap());
}