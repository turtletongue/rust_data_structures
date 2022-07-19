use std::error::Error;

use rust_data_structures::trees::{TraversingOrder, BinarySearchTree};

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

  tree.traverse(TraversingOrder::InOrder);

  Ok(())
}