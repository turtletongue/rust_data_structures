use rust_data_structures::search;

fn main() {
  let array = vec![1, 2, 3, 4, 5, 6, 7];

  println!("{}", search::exponential_search(&array, 7).unwrap());
}