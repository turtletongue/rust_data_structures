use std::error::Error;

use rust_data_structures::graphs::Graph;

fn main() -> Result<(), Box<dyn Error>> {
  let mut graph = Graph::new();

  let core = "Core";
  let console = "Console";
  let calculator = "Calculator";
  let utils = "Utils";

  graph.add_node(&core);
  graph.add_node(&console);
  graph.add_node(&calculator);
  graph.add_node(&utils);

  graph.add_edge(&utils, &core)?;
  graph.add_edge(&console, &core)?;
  graph.add_edge(&calculator, &console)?;
  graph.add_edge(&calculator, &utils)?;

  let sorted = graph.topological_sort();

  println!("{:#?}", sorted);

  Ok(())
}