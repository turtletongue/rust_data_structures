use std::error::Error;

use rust_data_structures::graphs::Graph;

fn main() -> Result<(), Box<dyn Error>> {
  let mut graph = Graph::new();

  graph.add_node("Anny");
  graph.add_node("Bob");
  graph.add_node("Tom");

  graph.add_edge(&"Anny", "Bob")?;
  graph.add_edge(&"Bob", "Anny")?;
  graph.add_edge(&"Bob", "Tom")?;

  graph.remove_edge(&"Anny", &"Bob");

  println!("{graph}");

  Ok(())
}