use std::error::Error;

use rust_data_structures::graphs::Graph;

fn main() -> Result<(), Box<dyn Error>> {
  let mut graph = Graph::new();

  let a = "A";
  let b = "B";
  let c = "C";
  let d = "D";

  graph.add_node(&a);
  graph.add_node(&b);
  graph.add_node(&c);
  graph.add_node(&d);

  graph.add_edge(&a, &b)?;
  graph.add_edge(&b, &c)?;
  graph.add_edge(&c, &a)?;
  graph.add_edge(&d, &a)?;

  println!("{:#?}", graph.has_cycle());

  Ok(())
}