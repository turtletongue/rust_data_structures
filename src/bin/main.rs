use std::error::Error;

use rust_data_structures::trees::BinarySearchTree;

fn main() -> Result<(), Box<dyn Error>> {
  let mut tree = BinarySearchTree::new();

  println!("{}", tree.find(7));

  tree.insert(7);
  tree.insert(4);
  tree.insert(9);
  tree.insert(1);
  tree.insert(6);
  tree.insert(8);
  tree.insert(10);
  tree.insert(15);
  tree.insert(7);
  tree.insert(7);

  println!("{}", tree.find(7));
  println!("{}", tree.find(15));

  Ok(())
}