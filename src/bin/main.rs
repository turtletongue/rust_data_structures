use std::error::Error;

use rust_data_structures::trees::{BinarySearchTree};

fn main() -> Result<(), Box<dyn Error>> {
  let mut tree1 = BinarySearchTree::new();

  tree1.insert(7);
  tree1.insert(4);
  tree1.insert(9);
  tree1.insert(1);
  tree1.insert(6);
  tree1.insert(8);
  tree1.insert(10);
  tree1.insert(15);

  let mut tree2 = BinarySearchTree::new();

  tree2.insert(7);
  tree2.insert(4);
  tree2.insert(9);
  tree2.insert(1);
  tree2.insert(6);
  tree2.insert(8);
  tree2.insert(10);
  tree2.insert(15);

  println!("{}", tree2.equals(&tree1));

  Ok(())
}