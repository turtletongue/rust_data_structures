use std::error::Error;

use rust_data_structures::hash_tables::HashTable;

fn main() -> Result<(), Box<dyn Error>> {
  let mut hash_table = HashTable::new(5);

  hash_table.put(6, String::from("Vec"));
  hash_table.put(6, String::from("Earth"));
  hash_table.put(1, String::from("Hi"));
  hash_table.put(3, String::from("Hello"));

  println!("{}", hash_table.get(6).unwrap());

  hash_table.remove(8);

  Ok(())
}