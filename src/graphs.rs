use std::{collections::{HashMap, HashSet}, hash::Hash, fmt::Display};

struct Node<'a, T> {
  value: &'a T,
  relatives: HashSet<&'a T>,
}

impl<'a, T: Eq + Hash> Node<'a, T> {
  pub fn new(value: &'a T) -> Self {
    Self {
      value,
      relatives: HashSet::new(),
    }
  }

  pub fn add_relative(&mut self, value: &'a T) {
    self.relatives.insert(value);
  }

  pub fn remove_relative(&mut self, value: &T) {
    if !self.has_relative(value) {
      return;
    }

    self.relatives.retain(|relative| *relative != value);
  }

  pub fn has_relative(&self, value: &T) -> bool {
    self.relatives.contains(value)
  }
}

impl<'a, T: Eq + Hash + Display> Display for Node<'a, T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.relatives.len() == 0 {
      return Ok(());
    }

    let mut formatted_relatives = String::new();

    for (index, relative) in self.relatives.iter().enumerate() {
      let delimeter = if index == self.relatives.len() - 1 { "" } else { ", " };

      formatted_relatives.push_str(&(relative.to_string() + delimeter));
    }

    let formatted_node = format!("{} is connected with [{}]", self.value.to_string(), formatted_relatives);

    writeln!(f, "{}", formatted_node)?;

    Ok(())
  }
}

pub struct Graph<'a, T> {
  nodes: HashMap<&'a T, Box<Node<'a, T>>>,
}

impl<'a, T: Eq + Hash + Clone> Graph<'a, T> {
  pub fn new() -> Self {
    Self {
      nodes: HashMap::new(),
    }
  }

  pub fn add_node(&mut self, value: &'a T) {
    self.nodes.entry(value).or_insert(Box::new(Node::new(value)));
  }

  pub fn remove_node(&mut self, value: &'a T) -> Result<(), &'static str> {
    if !self.is_node_exists(value) {
      return Err("Node is not exist");
    }

    for node in self.nodes.values_mut() {
      node.remove_relative(value);
    }

    self.nodes.remove(value);

    Ok(())
  }

  pub fn add_edge(&mut self, from: &T, to: &'a T) -> Result<(), &'static str> {
    if !self.is_node_exists(from) {
      return Err("From node is not exist");
    }

    if !self.is_node_exists(to) {
      return Err("To node is not exist");
    }

    let node = self.nodes.get_mut(from).unwrap();

    node.add_relative(to);

    Ok(())
  }

  pub fn remove_edge(&mut self, from: &T, to: &T) {
    if !self.is_node_exists(from) || !self.is_node_exists(to) {
      return;
    }

    let node = self.nodes.get_mut(from).unwrap();

    node.remove_relative(to);
  }

  fn is_node_exists(&self, value: &T) -> bool {
    self.nodes.get(value).is_some()
  }
}

impl<'a, T: Eq + Hash + Display> Display for Graph<'a, T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut formatted_graph = String::new();

    for node in self.nodes.values() {
      formatted_graph.push_str(&node.to_string());
    }

    write!(f, "{}", formatted_graph)?;
    
    Ok(())
  }
}