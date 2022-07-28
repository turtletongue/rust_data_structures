use std::{collections::{HashMap, BinaryHeap, HashSet}, hash::Hash, fmt::Display, cmp::{Ordering, Reverse}};

struct Edge {
  weight: usize,
}

impl Edge {
  pub fn new(weight: usize) -> Self {
    Self { weight }
  }
}

struct Node<'a, T> {
  value: &'a T,
  edges: HashMap<&'a T, Edge>,
}

impl<'a, T: Eq + Hash> Node<'a, T> {
  pub fn new(value: &'a T) -> Self {
    Self {
      value,
      edges: HashMap::new(),
    }
  }

  pub fn add_edge(&mut self, to: &'a T, weight: usize) {
    let edge = self.edges.entry(to).or_insert(Edge::new(weight));
    edge.weight = weight;
  }

}

impl<'a, T: Display> Display for Node<'a, T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.edges.len() == 0 {
      return Ok(());
    }

    let mut formatted_edges = String::new();

    for (index, (value, edge)) in self.edges.iter().enumerate() {
      let delimeter = if index == self.edges.len() - 1 { "" } else { ", " };

      formatted_edges.push_str(&(format!("{}({}){}", value, edge.weight, delimeter)));
    }

    let formatted_node = format!("{} is connected with [{}]", self.value.to_string(), formatted_edges);

    writeln!(f, "{}", formatted_node)?;

    Ok(())
  }
}

impl<'a, T: PartialEq> PartialEq for Node<'a, T> {
  fn eq(&self, other: &Self) -> bool {
    self.value == other.value
  }
}

impl<'a, T: PartialOrd> PartialOrd for Node<'a, T> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.value.partial_cmp(&other.value)
  }
}

impl<'a, T: Eq> Eq for Node<'a, T> {
  fn assert_receiver_is_total_eq(&self) {}
}

impl<'a, T: Ord> Ord for Node<'a, T> {
  fn cmp(&self, other: &Self) -> Ordering {
    self.value.cmp(other.value)
  }
}

pub struct WeightedGraph<'a, T> {
  nodes: HashMap<&'a T, Node<'a, T>>,
}

impl<'a, T: Eq + Hash + Ord> WeightedGraph<'a, T> {
  pub fn new() -> Self {
    Self {
      nodes: HashMap::new(),
    }
  }

  pub fn add_node(&mut self, value: &'a T) {
    self.nodes.insert(value, Node::new(value));
  }

  pub fn add_edge(&mut self, from: &'a T, to: &'a T, weight: usize) -> Result<(), &'static str>{
    if !self.is_node_exists(from) {
      return Err("From node is not exist");
    }

    if !self.is_node_exists(to) {
      return Err("To node is not exist");
    }

    self.nodes.get_mut(from).unwrap().add_edge(to, weight);
    self.nodes.get_mut(to).unwrap().add_edge(from, weight);

    Ok(())
  }

  pub fn shortest_path(&self, from: &'a T, to: &'a T) -> Result<Vec<&'a T>, &'static str> {
    let from_node = self.nodes.get(from);

    if from_node.is_none() {
      return Err("From node is not exist");
    }

    let to_node = self.nodes.get(to);

    if to_node.is_none() {
      return Err("To node is not exist");
    }

    let mut priority_queue = BinaryHeap::new();
    let mut distances = HashMap::new();
    let mut previous_nodes = HashMap::new(); 

    priority_queue.push(Reverse(from_node.unwrap()));
    distances.insert(from, 0);

    let mut visited_nodes = HashSet::with_capacity(self.nodes.len());

    while priority_queue.len() > 0 {
      let node_entry = priority_queue.pop();

      if node_entry.is_none() {
        continue;
      }

      let node = node_entry.unwrap().0;

      for (current, edge) in &node.edges {
        if visited_nodes.contains(current) {
          continue;
        }

        let proposed_distance = edge.weight + distances.get(node.value).or(Some(&0)).unwrap();
        let current_distance = distances.entry(current).or_insert(proposed_distance);
    
        if proposed_distance > *current_distance {
          continue;
        }
    
        *current_distance = proposed_distance;
        
        previous_nodes.insert(current, node.value);
        priority_queue.push(Reverse(&self.nodes[current]));
      }

      visited_nodes.insert(node.value);
    }

    Ok(self.construct_path(to_node, previous_nodes))
  }

  fn construct_path(&self, to_node: Option<&Node<'a, T>>, previous_nodes: HashMap<&&'a T, &'a T>) -> Vec<&'a T> {
    let mut result = Vec::new();
    let mut current = to_node;

    while current.is_some() {
      result.push(current.unwrap().value);

      let previous = previous_nodes.get(&current.unwrap().value);

      current = if let Some(previous) = previous {
        Some(&self.nodes[previous])
      } else {
        None
      }
    }
    
    result.iter().map(|value| *value).rev().collect()
  }

  fn is_node_exists(&self, value: &T) -> bool {
    self.nodes.contains_key(value)
  }
}

impl<'a, T: Eq + Hash + Display> Display for WeightedGraph<'a, T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut formatted_graph = String::new();

    for node in self.nodes.values() {
      formatted_graph.push_str(&node.to_string());
    }

    write!(f, "{}", formatted_graph)?;
    
    Ok(())
  }
}