use std::error::Error;

use rust_data_structures::avl_trees::AVLTree;

fn main() -> Result<(), Box<dyn Error>> {
  let mut tree = AVLTree::new();

  tree.insert(30);
  tree.insert(10);
  tree.insert(20);
  tree.insert(5);
  tree.insert(4);
  tree.insert(3);
  tree.insert(2);
  tree.insert(1);
  tree.insert(50);

  Ok(())
}