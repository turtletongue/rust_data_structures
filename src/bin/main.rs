use std::error::Error;

use rust_data_structures::weighted_graphs::WeightedGraph;

fn main() -> Result<(), Box<dyn Error>> {
  let mut graph = WeightedGraph::new();

  let a = "A";
  let b = "B";
  let c = "C";
  let d = "D";
  let e = "E";

  graph.add_node(&a);
  graph.add_node(&b);
  graph.add_node(&c);
  graph.add_node(&d);
  graph.add_node(&e);

  graph.add_edge(&a, &b, 1)?;
  graph.add_edge(&a, &d, 5)?;
  graph.add_edge(&a, &c, 1)?;
  graph.add_edge(&b, &e, 4)?;
  graph.add_edge(&d, &e, 1)?;
  graph.add_edge(&c, &d, 1)?;
  graph.add_edge(&b, &d, 4)?;

  println!("{:#?}", graph.has_cycle());

  Ok(())
}