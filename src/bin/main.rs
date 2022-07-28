use std::error::Error;

use rust_data_structures::weighted_graphs::WeightedGraph;

fn main() -> Result<(), Box<dyn Error>> {
  let mut graph = WeightedGraph::new();

  let a = "A";
  let b = "B";
  let c = "C";
  let d = "D";

  graph.add_node(&a);
  graph.add_node(&b);
  graph.add_node(&c);
  graph.add_node(&d);
  
  graph.add_edge(&a, &b, 3)?;
  graph.add_edge(&b, &d, 4)?;
  graph.add_edge(&a, &c, 1)?;
  graph.add_edge(&c, &b, 2)?;
  graph.add_edge(&c, &d, 5)?;

  println!("{}", graph.minimum_spanning_tree()?);

  Ok(())
}