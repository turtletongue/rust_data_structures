use std::error::Error;

use rust_data_structures::graphs::Graph;

fn main() -> Result<(), Box<dyn Error>> {
  let mut graph = Graph::new();

  let anny = "Anny";
  let bob = "Bob";
  let tom = "Tom";

  graph.add_node(&anny);
  graph.add_node(&bob);
  graph.add_node(&tom);

  graph.add_edge(&anny, &bob)?;
  graph.add_edge(&bob, &anny)?;
  graph.add_edge(&bob, &tom)?;

  graph.remove_edge(&anny, &bob);

  println!("{graph}");

  Ok(())
}