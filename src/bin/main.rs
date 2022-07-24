use std::error::Error;

use rust_data_structures::tries::Trie;

fn main() -> Result<(), Box<dyn Error>> {
  let mut trie = Trie::new();

  trie.insert("car");
  trie.insert("card");
  trie.insert("careful");
  trie.insert("egg");

  println!("{:#?}", trie.complete("car"));

  Ok(())
}