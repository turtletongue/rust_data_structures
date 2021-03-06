use std::{cell::RefCell, rc::Rc, cmp::{Ord, Ordering}, fmt::Display};

type OptionalNode<T> = Option<Rc<RefCell<Node<T>>>>;
type OptionalNodeRef<'a, T> = Option<&'a Rc<RefCell<Node<T>>>>;

struct Node<T> {
  value: T,
  left_child: OptionalNode<T>,
  right_child: OptionalNode<T>,
}

impl<T> Node<T> {
  pub fn new(value: T) -> Self {
    Self {
      value,
      left_child: None,
      right_child: None,
    }
  }
}

pub enum TraversingOrder {
  PreOrder,
  InOrder,
  PostOrder,
  LevelOrder,
}

pub struct BinarySearchTree<T> {
  root: OptionalNode<T>,
}

impl<T: Ord + Display + Eq + Clone> BinarySearchTree<T> {
  pub fn new() -> Self {
    Self {
      root: None,
    }
  }

  pub fn insert(&mut self, value: T) {
    let parent = self.find_free_parent(&value);

    if parent.is_none() {
      if self.root.is_none() {
        self.root = Self::create_optional_node(value);
      }

      return;
    }
    
    let mut parent_borrowed = parent.as_ref().unwrap().borrow_mut();
    let value_order = parent_borrowed.value.cmp(&value);

    let node = Self::create_optional_node(value);
    
    if value_order == Ordering::Greater {
      parent_borrowed.left_child = node;
    } else {
      parent_borrowed.right_child = node;
    };
  }

  pub fn find(&self, value: T) -> bool {
    self.root.is_some() && self.find_free_parent(&value).is_none()
  }

  pub fn traverse(&self, order: TraversingOrder) {
    let root = self.root.as_ref();

    match order {
      TraversingOrder::PreOrder => Self::traverse_pre_order(root),
      TraversingOrder::InOrder => Self::traverse_in_order(root),
      TraversingOrder::PostOrder => Self::traverse_post_order(root),
      TraversingOrder::LevelOrder => self.traverse_level_order(),
    }
  }

  pub fn get_height(&self) -> i32 {
    Self::height(self.root.as_ref())
  }

  pub fn equals(&self, other: &Self) -> bool {
    Self::is_equal(self.root.as_ref(), other.root.as_ref())
  }

  pub fn validate(&self) -> bool {
    Self::is_valid(self.root.as_ref(), None, None)
  }

  pub fn get_values_at_distance(&self, distance: usize) -> Vec<T> {
    let mut result = Vec::new();

    Self::collect_values_at_distance(self.root.as_ref(), distance, &mut result);

    result
  }

  fn traverse_level_order(&self) {
    let height = self.get_height();

    if height < 0 {
      return;
    }

    for level in 0..=height {
      for value in self.get_values_at_distance(level.try_into().unwrap()) {
        println!("{value}");
      }
    }
  }

  fn traverse_pre_order(root: OptionalNodeRef<T>) {
    if root.is_none() {
      return;
    }

    let borrowed_root = root.unwrap().borrow();

    println!("{}", borrowed_root.value);

    Self::traverse_pre_order(borrowed_root.left_child.as_ref());

    Self::traverse_pre_order(borrowed_root.right_child.as_ref());
  }

  fn traverse_in_order(root: OptionalNodeRef<T>) {
    if root.is_none() {
      return;
    }

    let borrowed_root = root.unwrap().borrow();

    Self::traverse_in_order(borrowed_root.left_child.as_ref());

    println!("{}", borrowed_root.value);

    Self::traverse_in_order(borrowed_root.right_child.as_ref());
  }

  fn traverse_post_order(root: OptionalNodeRef<T>) {
    if root.is_none() {
      return;
    }

    let borrowed_root = root.unwrap().borrow();

    Self::traverse_post_order(borrowed_root.left_child.as_ref());

    Self::traverse_post_order(borrowed_root.right_child.as_ref());

    println!("{}", borrowed_root.value);
  }

  fn height(root: OptionalNodeRef<T>) -> i32 {
    if root.is_none() {
      return -1;
    }

    let borrowed_root  = root.unwrap().borrow();

    if borrowed_root.left_child.is_none() && borrowed_root.right_child.is_none() {
      return 0;
    }

    let left_subtree_height = Self::height(borrowed_root.left_child.as_ref());
    let right_subtree_height = Self::height(borrowed_root.right_child.as_ref());

    1 + left_subtree_height.max(right_subtree_height)
  }

  fn is_equal(root: OptionalNodeRef<T>, other: OptionalNodeRef<T>) -> bool {
    if root.is_none() || other.is_none() {
      return root.is_none() && other.is_none();
    }

    let borrowed_root = root.unwrap().borrow();
    let borrowed_other = other.unwrap().borrow();

    let root_left = borrowed_root.left_child.as_ref();
    let other_left = borrowed_other.left_child.as_ref();

    let root_right = borrowed_root.right_child.as_ref();
    let other_right = borrowed_other.right_child.as_ref();

    borrowed_root.value == borrowed_other.value &&
      Self::is_equal(root_left, other_left) &&
      Self::is_equal(root_right, other_right)
  }

  fn is_valid(root: OptionalNodeRef<T>, min: Option<&T>, max: Option<&T>) -> bool {
    if root.is_none() {
      return true;
    }

    let borrowed_root = root.unwrap().borrow();

    let root_value = &borrowed_root.value;

    let is_greater_then_min = min.is_none() || min.unwrap().cmp(root_value) == Ordering::Less;
    let is_less_then_max = max.is_none() || max.unwrap().cmp(root_value) == Ordering::Greater;

    let left = borrowed_root.left_child.as_ref();
    let right = borrowed_root.right_child.as_ref();

    is_greater_then_min && is_less_then_max &&
      Self::is_valid(left, min, Some(root_value)) &&
      Self::is_valid(right, Some(root_value), max)
  }

  fn collect_values_at_distance(root: OptionalNodeRef<T>, distance: usize, list: &mut Vec<T>) {
    if root.is_none() {
      return;
    }

    let borrowed_root = root.unwrap().borrow();

    if distance == 0 {
      return list.push(borrowed_root.value.clone());
    }

    Self::collect_values_at_distance(borrowed_root.left_child.as_ref(), distance - 1, list);
    Self::collect_values_at_distance(borrowed_root.right_child.as_ref(), distance - 1, list);
  }

  fn find_free_parent(&self, value: &T) -> OptionalNode<T> {
    if self.root.is_none() {
      return None;
    }

    let mut current = Rc::clone(self.root.as_ref().unwrap());

    loop {
      current = {
        let node_borrowed = current.borrow();

        let next = match node_borrowed.value.cmp(&value) {
          Ordering::Greater => node_borrowed.left_child.as_ref(),
          Ordering::Less => node_borrowed.right_child.as_ref(),
          Ordering::Equal => return None,
        };

        if next.is_none() {
          break;
        }

        Rc::clone(next.unwrap())
      };
    }

    Some(current)
  }

  fn create_optional_node(value: T) -> OptionalNode<T> {
    Some(Rc::new(RefCell::new(Node::new(value))))
  }
}