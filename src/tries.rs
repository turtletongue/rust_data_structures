use std::collections::HashMap;

struct Node {
  value: char,
  children: HashMap<char, Box<Node>>,
  is_end_of_word: bool,
}

impl Node {
  pub fn new(value: char, is_end_of_word: bool) -> Self {
    Self {
      value,
      children: HashMap::new(),
      is_end_of_word,
    }
  }

  pub fn insert_child(&mut self, child: char, is_end_of_word: bool) -> &mut Box<Self> {
    self.children
      .entry(child)
      .or_insert(Box::new(Self::new(child, is_end_of_word)))
  }

  pub fn remove_child(&mut self, child: char) {
    self.children.remove(&child);
  }

  pub fn get_child(&self, child: char) -> Option<&Box<Self>> {
    self.children.get(&child)
  }

  pub fn get_child_mut(&mut self, child: char) -> Option<&mut Box<Self>> {
    self.children.get_mut(&child)
  }

  pub fn get_children(&self) -> Vec<&Box<Node>> {
    self.children.values().collect()
  }

  pub fn has_children(&self) -> bool {
    self.children.len() != 0
  }
}

pub struct Trie {
  root: Box<Node>,
}

impl Trie {
  pub fn new() -> Self {
    Self {
      root: Box::new(
        Node::new(char::default(), false)
      ),
    }
  }

  pub fn insert(&mut self, word: &str) {
    if word.len() == 0 {
      return;
    }

    Self::insert_recursive(&mut self.root, word);
  }

  pub fn contains(&self, word: &str) -> bool {
    Self::contains_recursive(&self.root, word)
  }

  pub fn remove(&mut self, word: &str) {
    Self::remove_recursive(&mut self.root, word);
  }

  pub fn complete(&self, prefix: &str) -> Vec<String> {
    let mut result = Vec::new();

    let word = if prefix.len() > 0 { prefix.chars().take(prefix.len() - 1).collect() } else { String::from(prefix) };

    Self::positionate_completion(&self.root, prefix, word, &mut result);

    result
  }

  fn insert_recursive(current: &mut Box<Node>, word: &str) {
    let next_char = word.chars().next().unwrap();
    let is_end_of_word = word.chars().count() == 1;

    let next = current.insert_child(next_char.to_lowercase().next().unwrap(), is_end_of_word);

    if is_end_of_word {
      return;
    }

    let word_tail: String = word.chars().into_iter().skip(1).collect();

    Self::insert_recursive(next, &word_tail);
  }

  fn contains_recursive(current: &Box<Node>, word: &str) -> bool {
    let next_char = word.chars().next();

    if next_char.is_none() {
      return current.is_end_of_word;
    }

    let next = current.get_child(next_char.unwrap().to_lowercase().next().unwrap());

    if next.is_none() {
      return false;
    }

    let word_tail: String = word.chars().into_iter().skip(1).collect();

    Self::contains_recursive(next.unwrap(), &word_tail)
  }

  fn remove_recursive(current: &mut Box<Node>, word: &str) {
    let next_char = word.chars().next();

    if next_char.is_none() {
      current.is_end_of_word = false;

      return;
    }

    let next_char = next_char.unwrap();

    let next = current.get_child_mut(next_char.to_lowercase().next().unwrap());

    if next.is_none() {
      return;
    }

    let next = next.unwrap();

    let word_tail: String = word.chars().into_iter().skip(1).collect();

    Self::remove_recursive(next, &word_tail);

    if !next.is_end_of_word && !next.has_children() {
      current.remove_child(next_char);
    }
  }

  fn positionate_completion(current: &Box<Node>, prefix: &str, word: String, result: &mut Vec<String>) {
    let next_char = prefix.chars().next();

    if next_char.is_none() {
      Self::complete_recursive(Some(current), word.clone(), result);

      return;
    }

    let next = current.get_child(next_char.unwrap().to_lowercase().next().unwrap());

    if next.is_none() {
      return;
    }

    let word_tail: String = prefix.chars().into_iter().skip(1).collect();

    Self::positionate_completion(next.unwrap(), &word_tail, word, result);
  }

  fn complete_recursive(current: Option<&Box<Node>>, word: String, result: &mut Vec<String>) {
    if current.is_none() {
      return;
    }

    let current = current.unwrap();

    let mut word = word;

    if current.value != char::default() {
      word.push(current.value);
    }

    if current.is_end_of_word {
      result.push(word.clone());
    }

    for child in current.get_children() {
      Self::complete_recursive(Some(child), word.clone(), result)
    }
  }
}