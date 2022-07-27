use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
struct Edge {
  weight: usize,
}

impl Edge {
  pub fn new(weight: usize) -> Self {
    Self { weight }
  }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct WeightedGraph<'a, T> {
  nodes: HashMap<&'a T, Node<'a, T>>,
}

impl<'a, T: Eq + Hash> WeightedGraph<'a, T> {
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

  fn is_node_exists(&self, value: &T) -> bool {
    self.nodes.contains_key(value)
  }
}