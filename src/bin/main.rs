use std::error::Error;

use rust_data_structures::weighted_graphs::WeightedGraph;

fn main() -> Result<(), Box<dyn Error>> {
  let mut graph = WeightedGraph::new();

  let alice = "Alice";
  let bob = "Bob";
  let john = "John";

  graph.add_node(&alice);
  graph.add_node(&bob);
  graph.add_node(&john);

  graph.add_edge(&alice, &john, 1)?;
  graph.add_edge(&bob, &john, 5)?;
  graph.add_edge(&john, &bob, 7)?;

  println!("{:#?}", graph);

  Ok(())
}